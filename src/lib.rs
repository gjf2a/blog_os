#![no_std]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

use pc_keyboard::DecodedKey;
use core::panic::PanicInfo;

pub fn init(timer_handler: fn(), keyboard_handler: fn(DecodedKey)) {
    gdt::init();
    interrupts::init_idt(timer_handler, keyboard_handler);
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}