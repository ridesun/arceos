use arceos_api::stdio::ax_console_write_fmt;
use core::ffi::{c_char, VaList};

pub(crate) unsafe extern "C" fn abi_printf(str_ptr: *const c_char, args: VaList) {
    let format = printf_compat::output::display(str_ptr, args);
    ax_console_write_fmt(format_args!("{}\n", format)).unwrap();
}
pub(crate) fn abi_putchar(c: char) {
    ax_console_write_fmt(format_args!("{}", c)).unwrap()
}