#ifndef _LIBC_H
#define _LIBC_H

#define hidden __attribute__((visibility("hidden")))

typedef signed char     int8_t;
typedef signed short    int16_t;
typedef signed int      int32_t;
typedef unsigned char   uint8_t;
typedef unsigned short  uint16_t;
typedef unsigned int    uint32_t;
typedef unsigned socklen_t;

#define SYS_PUTCHAR 1
#define SYS_PRINTF 2
#define SYS_TIMESPEC 3

void abi_call(unsigned long entry,int abi_id,long arg);
//extern int main(int, char **);
extern int main();
#endif
