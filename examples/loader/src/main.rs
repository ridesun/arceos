#![feature(asm_const)]
#![feature(c_variadic)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use axstd::os::arceos::modules::axhal::time::monotonic_time;
use axstd::os::arceos::modules::axlog::{debug, info};
use axstd::print;
#[cfg(feature = "axstd")]
use axstd::println;
use core::ffi::VaList;
use core::slice::from_raw_parts;
use core::{mem::size_of, usize};
use elf::endian::AnyEndian;
use elf::ElfBytes;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const HEADER_SIZE: usize = 64;
const RUN_START: usize = 0xffff_ffc0_8010_0000;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
const SYS_TIMESPEC: usize = 4;
const SYS_PRINTF: usize = 5;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

unsafe extern "C" fn c_print(str_ptr: *const u8, args: VaList) {
    let format = printf_compat::output::display(str_ptr, args);
    println!("{}", format);
}
fn abi_putchar(c: char) {
    print!("{}", c);
}

fn abi_terminate() {
    println!("[ABI:Exit] exit");
    axstd::process::exit(0);
}

#[repr(C)]
#[derive(Debug)]
struct TimeSpec {
    tv_sec: usize,
    tv_nsec: usize,
}
fn abi_timespec(ts: *mut TimeSpec) {
    unsafe {
        let ts = &mut *ts;
        let now = monotonic_time();
        ts.tv_nsec = now.as_nanos() as usize;
        ts.tv_sec = now.as_secs() as usize;
        debug!("{:?}", ts);
    }
}
#[repr(C)]
pub struct AppHeader {
    magic: u64,
    app_count: u64,
    app_size: [u64; 6],
}

impl AppHeader {
    pub const MAGIC: u64 = 0x4150505F;

    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC
    }
    pub fn from_bytes(data: &[u8]) -> Option<&Self> {
        if data.len() < size_of::<Self>() {
            return None;
        }
        let header = unsafe { &*(data.as_ptr() as *const Self) };
        if !header.is_valid() {
            return None;
        }
        Some(header)
    }
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let mut run_entry = 0usize;
    let header_array = unsafe { from_raw_parts(PLASH_START as *const u8, HEADER_SIZE) };
    if let Some(header) = AppHeader::from_bytes(header_array) {
        let mut start = PLASH_START + HEADER_SIZE;
        let mut run_start = RUN_START;
        for i in header.app_size {
            if i != 0 {
                let apps_start = start as *const u8;
                debug!("apps_start: {:#X?},size:{}", apps_start, i);
                let elf_slice = unsafe { from_raw_parts(apps_start, i as usize) };
                let elf =
                    ElfBytes::<AnyEndian>::minimal_parse(elf_slice).expect("Failed to parse ELF");
                let elf_ehdr = elf.ehdr;
                debug!("elf ehdr:{:?}", elf_ehdr);

                match elf_ehdr.e_type {
                    elf::abi::ET_EXEC => {
                        println!("load static application");
                        let dot_text = elf
                            .section_header_by_name(".text")
                            .expect("section table should be parseable")
                            .expect("file should have a .text section");
                        let dot_text_slice = elf_slice
                            .get(dot_text.sh_offset as usize..)
                            .expect("Invalid .text section");
                        let run_code = unsafe {
                            core::slice::from_raw_parts_mut(
                                run_start as *mut u8,
                                dot_text_slice.len(),
                            )
                        };
                        run_code.copy_from_slice(&dot_text_slice);
                        debug!(
                            "run code at:{:#X?},size:{}",
                            run_start,
                            dot_text_slice.len()
                        );
                        // run_entry = elf_ehdr.e_entry;
                        run_entry = RUN_START;
                        run_start += dot_text_slice.len();
                    }
                    elf::abi::ET_DYN => {}
                    _ => {
                        panic!("Invalid ELF type")
                    }
                };

                start += i as usize;
            }
        }
    } else {
        panic!("Failed to parse Header");
    }

    println!("Load payload ok!");

    println!("Execute app ...");

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);
    register_abi(SYS_TIMESPEC, abi_timespec as usize);
    register_abi(SYS_PRINTF, c_print as usize);

    // execute app
    unsafe {
        core::arch::asm!("
        la      a7, {abi_table}
        mv      t2, {run_start}
        jalr    t2
        j       .",
        run_start = in(reg) run_entry,
        abi_table = sym ABI_TABLE,
        )
    }
}
