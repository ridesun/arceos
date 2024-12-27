#include <stdio.h>
#include <libc.h>
#include <stdarg.h>

extern unsigned long volatile abi_entry;
void printf(const char *restrict fmt,...){
	va_list ap;
	va_start(ap, fmt);
    typedef void (*FnABI)(const char*,va_list);
    long *abi_ptr=(long *)(abi_entry +  8 * SYS_PRINTF);
    FnABI func = (FnABI)(*abi_ptr);
    va_list *ap_ptr=&ap;
    if (func!=0x0){
        func(fmt,ap);
    }
	va_end(ap);
}