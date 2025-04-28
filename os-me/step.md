rustc --version --verbose
cargo run --target riscv64gc-unknown-none-elf
cargo build --target riscv64gc-unknown-none-elf

file target/riscv64gc-unknown-none-elf/debug/os-me
rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os-me
rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os-me


# new
cargo build --release

rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/example-test --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin

# 普通qemu
qemu-system-riscv64 -machine virt -nographic -bios bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
# -s 相当于 -gdb tcp::1234
qemu-system-riscv64 -machine virt -nographic -bios bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 -s -S 

rust-gdb target/riscv64gc-unknown-none-elf/release/os.bin


# in gdb
set architecture riscv:rv64
target remote :1234
b _start
c

info symbols

disassemble _start

file target/riscv64gc-unknown-none-elf/debug/example-test
