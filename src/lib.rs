#![feature(abi_x86_interrupt)]
#![no_std]
use core::fmt;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;


pub fn init() {
	gdt::init_gdt();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}


#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => {$crate::print!('\n')};
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
