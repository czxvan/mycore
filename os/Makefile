
os: src/*
	cargo build --release

os.bin: os
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

run: os os.bin
	../run.sh

.PHONY: all os os.bin