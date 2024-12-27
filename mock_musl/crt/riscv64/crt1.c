#include <libc.h>
#include "../../arch/riscv64/crt_arch.h"

unsigned long volatile abi_entry=0;

void terminate();
hidden void _start_c(long *p)
{
    __asm__ volatile (
    "mv %0, a7"  // 将 a7 的值移动到 abi_entry
    : "=r" (abi_entry)  // 输出操作数
    );
    int argc = p[0];
    char **argv = (void *)(p+1);

//    main(argc, argv);
    main();
    
}

