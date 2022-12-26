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
	use core::fmt::Write;
	vga_buffer::WRITER.lock().write_str("Hello, World!\n").unwrap();
	write!(vga_buffer::WRITER.lock(), "\nSome numbers: {} {}", 42, 1.337).unwrap();
	
	loop {}
}
