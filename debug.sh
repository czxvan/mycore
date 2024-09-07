riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234' \
    -ex 'b *0x80200000' \
    -ex 'c' \
    -ex 'x/10i $pc'