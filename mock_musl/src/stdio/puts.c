#include <stdio.h>
#include <libc.h>

extern unsigned long volatile abi_entry; 

void putchar(char c){
  abi_call(abi_entry,SYS_PUTCHAR,c-0);
}
void puts(char* s){
  while (*s !='\0'){
    putchar(*s);
    s++;
  }
  putchar('\n');
}

