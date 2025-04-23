rustc --version --verbose
cargo run --target riscv64gc-unknown-none-elf
cargo build --target riscv64gc-unknown-none-elf

file target/riscv64gc-unknown-none-elf/debug/os-me
rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os-me
rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os-me

