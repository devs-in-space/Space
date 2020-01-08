#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
extern crate alloc;
use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
 	panic!("Ded")
}

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use alloc::{vec::Vec, vec};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	let i: usize = 1000*2048;
	 space::init();
	 space::println!("Hola Bish");
	unsafe {
		ALLOCATOR.lock().init(boot_info.physical_memory_offset as usize, i)
	}



	space::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	space::println!("{}", _info);
    space::hlt_loop();
}


