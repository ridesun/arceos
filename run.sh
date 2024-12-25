#!/bin/bash
CRTDIR=$(pwd)
cd $CRTDIR/mock_musl
make
cd $CRTDIR/arceos_tool
cargo build --release
mv ./target/release/arceos_tool "$CRTDIR"/hello_c
cd $CRTDIR/hello_c
bash build.sh
cd $CRTDIR
make run LOG=debug ARCH=riscv64 A=examples/loader