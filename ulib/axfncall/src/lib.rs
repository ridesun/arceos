#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(c_variadic)]

use arceos_api::modules::axlog::info;
const SYS_UNDFINEDFN: usize = 0;
const SYS_PUTCHAR: usize = 1;
const SYS_PRINTF: usize = 2;
const SYS_TIMESPEC: usize = 3;

#[cfg(feature = "time")]
pub(crate) mod time;

#[cfg(feature = "time")]
use crate::time::abi_timespec;

#[cfg(feature = "stdio")]
pub(crate) mod stdio;
#[cfg(feature = "stdio")]
use stdio::{abi_printf, abi_putchar};
pub static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}
fn un_defined_function(abi_id: usize) {
    info!("Call undefined function,abi id:{}", abi_id);
}
pub fn init() {
    register_abi(SYS_UNDFINEDFN, un_defined_function as usize);
    #[cfg(feature = "time")]{
        register_abi(SYS_TIMESPEC, abi_timespec as usize);
    }
    #[cfg(feature = "stdio")]{
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_PRINTF, abi_printf as usize);
    }
}