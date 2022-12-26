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
	
	vga_buffer::WRITER.lock().set_fg_color(vga_buffer::Color::White);
	writeln!(vga_buffer::WRITER.lock(), "Some numbers: {} {}", 42, 1.337).unwrap();

	vga_buffer::WRITER.lock().set_colors(vga_buffer::Color::DarkGray, vga_buffer::Color::White);
	writeln!(vga_buffer::WRITER.lock() ,"Testing! 1..2..3..").unwrap();

	vga_buffer::WRITER.lock().set_bg_color(vga_buffer::Color::Black);
	write!(vga_buffer::WRITER.lock() , "0123456789").unwrap();
	
	loop {}
}
