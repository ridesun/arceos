#![no_std]
#![no_main]

mod abi;

#[cfg(not(test))]
use core::panic::PanicInfo;

use abi::{ABI_TABLE_ADDR, init_abis};
pub use abi::{
	exit,
	putchar,
	puts,
};

unsafe extern "C" {
    fn main();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() {
    unsafe {
        core::arch::asm!("
            mv      {abi_table}, a7",
            abi_table = out(reg) ABI_TABLE_ADDR,
        );
        init_abis();

        main();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}