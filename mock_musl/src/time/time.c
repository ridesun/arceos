#include <time.h>
#include <libc.h>
#include <stdio.h>
#include "../arch/riscv64/bits/limits.h"
extern unsigned long volatile abi_entry;
#define SYS_TIMESPEC 4
clock_t clock()
{
	struct timespec ts;
    struct timespec *ts_ptr=&ts;
	abi_call(abi_entry,SYS_TIMESPEC,(long)ts_ptr);

	if (ts.tv_sec > LONG_MAX/1000000
	 || ts.tv_nsec/1000 > LONG_MAX-1000000*ts.tv_sec)
		return -1;

	return ts.tv_sec*1000000 + ts.tv_nsec/1000;
}