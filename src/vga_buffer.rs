
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black		= 0,
	Blue		= 1,
	Green		= 2,
	Cyan		= 3,
	Red			= 4,
	Magenta		= 5,
	Brown		= 6,
	LightGray	= 7,
	DarkGray	= 8,
	LightBlue	= 9,
	LightGreen	= 10,
	LightCyan	= 11,
	LightRed	= 12,
	Pink		= 13,
	Yellow		= 14,
	White		= 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
	fn bg(&mut self, col: Color) {
		// BIT MANIPULATION!!!
		// 0b11110000 mask for bg (0xF0)
		self.0 = (0xF0 & ((col as u8) << 4)) | 0x0F & self.0;
	}

	fn fg(&mut self, col: Color) {
		// MORE BIT MANIPULATION!!!
		// 0b00001111 mask for fg (0x0F)
		self.0 = 0xF0 & self.0 | 0x0F & col as u8;
	}
	
	fn new(foreground: Color, background: Color) -> ColorCode {
		let mut col = ColorCode(0);
		col.bg(background);
		col.fg(foreground);
		col
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	col_position: usize,
	row_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.col_position >= BUFFER_WIDTH {
					self.new_line();
				}

				if self.row_position >= BUFFER_HEIGHT {
					self.row_position = 0;
				}
				
				let color_code = self.color_code;
				self.buffer.chars[self.row_position][self.col_position].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.col_position += 1;
			}
		}
	}
	
	fn new_line(&mut self) {
		self.col_position = 0;
		self.row_position += 1;
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
	}
}

use core::fmt;
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

use core::fmt::Write;

pub fn print_something() {
	let mut writer = Writer {
		row_position: BUFFER_WIDTH - 1,
		col_position: 0,
		color_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writeln!(writer, "Hello, World!!").unwrap();
	
	writer.color_code.bg(Color::Black);
	writer.color_code.fg(Color::White);
	let bg = (writer.color_code.0 & 0xF0) >> 4;
	let fg = writer.color_code.0 & 0x0F;
	write!(writer, "Color data bg: {} fg: {}", bg, fg).unwrap();
}
