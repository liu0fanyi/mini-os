#![no_std]
#![no_main]

use core::panic::PanicInfo;

use axhal::shutdown;
use axlog::*;
use axstd::println;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub fn rust_main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    //
    //
    unsafe extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    println!("Hello, world!");
    axlog::init();

    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    println!("where?, world!");
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    // shutdown()
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message());
    }
    shutdown()
}

// on bash: qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os-me; echo $?
