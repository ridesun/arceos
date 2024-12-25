#include <stdio.h>
#include <libc.h>
#include <stdarg.h>
#define SYS_PRINT 5
extern unsigned long volatile abi_entry;
void printf(const char *restrict fmt,...){
	va_list ap;
	va_start(ap, fmt);
    typedef void (*FnABI)(const char*,va_list);
    long *abi_ptr=(long *)(abi_entry +  8 * SYS_PRINT);
    FnABI func = (FnABI)(*abi_ptr);
    va_list *ap_ptr=&ap;
    func(fmt,ap);
	va_end(ap);
}