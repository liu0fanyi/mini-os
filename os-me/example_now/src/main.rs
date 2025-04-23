#![no_std]
#![no_main]

use core::panic::PanicInfo;

use axhal::sys_exit;
use axstd::println;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello, world!");
    sys_exit(9);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// on bash: qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os-me; echo $?
