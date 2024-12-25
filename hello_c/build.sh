#! /bin/bash
riscv64-linux-musl-gcc  -nostdlib -nostartfiles -ffreestanding -O2 -mcmodel=medany -I../mock_musl/include -c hello.c
riscv64-linux-musl-ld  ../mock_musl/crt/riscv64/crt1.o hello.o ../mock_musl/libmock.a -T../mock_musl/ld/riscv64.ld -o hello
# riscv64-linux-musl-strip -s hello
riscv64-linux-musl-objcopy hello ./hello_app.bin
./arceos_tool
mv apps.bin ../payload/
