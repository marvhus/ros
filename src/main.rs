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
	use vga_buffer::{Color, set_fg_color, set_bg_color, set_colors, WRITER};
	
	WRITER.lock().write_str("Hello, World!\n").unwrap();
	
	set_fg_color(vga_buffer::Color::White);
	writeln!(WRITER.lock(), "Some numbers: {} {}", 42, 1.337).unwrap();

	set_colors(Color::DarkGray, vga_buffer::Color::White);
	writeln!(WRITER.lock() ,"Testing! 1..2..3..").unwrap();

	set_bg_color(Color::Black);
	write!(WRITER.lock() , "0123456789").unwrap();
	
	loop {}
}
