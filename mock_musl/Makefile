CROSS_COMPILE ?= riscv64-linux-musl-
CC = $(CROSS_COMPILE)gcc
AS = $(CROSS_COMPILE)as
LD = $(CROSS_COMPILE)ld
AR = $(CROSS_COMPILE)ar

CFLAGS = -ffreestanding -nostdlib -O2 -mcmodel=medany -I./include
ASFLAGS = 
LDFLAGS = -T ldscripts/riscv64.ld

LIBOBJS = src/stdio/puts.o \
          		  src/libc/abi.o \
          		  src/time/time.o \
          		  src/stdio/printf.o \
          		  src/stdio/perror.o \
          		  src/network/inet_pton.o
          
CRTOBJS = crt/riscv64/crt1.o

DIRS = stdio \
       crt/riscv64

.PHONY: all

all: libmock.a $(CRTOBJS)

$(DIRS):

	mkdir -p $@


$(LIBOBJS): | $(dir $(LIBOBJS))
$(CRTOBJS): | $(dir $(CRTOBJS))

crt/riscv64/crt1.o: crt/riscv64/crt1.c arch/riscv64/crt_arch.h
	$(CC) $(CFLAGS) -c -o $@ $<

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $<

libmock.a: $(LIBOBJS)
	$(AR) rcs $@ $(LIBOBJS)

