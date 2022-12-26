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
	use vga_buffer::{Color, set_fg_color, set_bg_color, set_colors};
	
	println!("Hello, World!");
	
	set_fg_color(Color::White);
	println!("Some numbers: {} {}", 42, 1.337);

	set_colors(Color::DarkGray, Color::White);
	println!("Testing! 1..2..3..");

	set_bg_color(Color::Black);
	println!("0123456789");

	set_fg_color(Color::White);
	print!("Test asd ");
	println!("Hello");
	print!("sdfjklsdfj");
	
	loop {}
}
