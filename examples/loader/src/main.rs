#![no_std]
#![no_main]
    
use axlog::info;
use axstd::println;

mod abi;
use abi::{init_abis, ABI_TABLE};

mod load;
use load::load_elf;

#[unsafe(no_mangle)]
fn main() {
    init_abis();

    let entry = load_elf();

	println!("Entry {:?}", entry);

    info!("Execute app ...");
    unsafe {
        core::arch::asm!("
			la      a7, {abi_table}
			mv      t2, {entry}
			jalr    t2",
            entry = in(reg) entry,
            abi_table = sym ABI_TABLE,
        )
    }
}
