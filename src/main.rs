#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {} 
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	vga_buffer::print_something();
	
	loop {}
}
