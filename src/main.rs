#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga_buffer::{Color, set_fg_color, set_bg_color, set_colors};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	set_colors(Color::Red, Color::Black);
	println!("{}", info);
	loop {} 
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	
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
	println!("sdfjklsdfj");

	//panic!("Testing 1.. 2.. 3..");
	
	loop {}
}
