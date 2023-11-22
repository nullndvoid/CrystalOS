use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::slice::from_mut;
use crate::kernel::render::{ColorCode, ScreenChar};
use crate::{printerr, serial_println};
use crate::std::frame::special_char;
use crate::std::io::Color;
use crate::user::lib::libgui::cg_core::XorY::Both;

#[derive(Copy, Clone, Debug)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

impl Position {
	pub fn new(x: usize, y: usize) -> Position {
		Position { x, y }
	}
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum XorY {
	X,
	Y,
	Both,
	None,
}


impl XorY {
	pub fn setx(&mut self) {
		if self == &XorY::None {
			*self = XorY::X;
		} else if self == &XorY::Y {
			*self = XorY::Both;
		}
	}
	pub fn sety(&mut self) {
		if self == &XorY::None {
			*self = XorY::Y;
		} else if self == &XorY::X {
			*self = XorY::Both;
		}
	}
}


#[derive(Debug)]
pub enum GuiError {
	OutOfBounds(XorY)
}

pub type Dimensions = Position;

#[derive(Clone, Copy, Debug)]
pub struct ColouredChar {
	pub character: char,
	pub colour: ColorCode,
}

impl ColouredChar {
	pub fn new(character: char, colour: ColorCode) -> ColouredChar {
		ColouredChar {
			character,
			colour,
		}
	}
	pub fn white(character: char) -> ColouredChar {
		ColouredChar {
			character,
			colour: ColorCode::new(Color::White, Color::Black),
		}
	}
	pub fn null() -> ColouredChar {
		ColouredChar {
			character: ' ',
			colour: ColorCode::new(Color::White, Color::Black),
		}
	}
	pub fn as_screen_char(&self) -> ScreenChar {
		ScreenChar {
			character: {
				if let Some(c) = special_char(self.character) {
					c
				} else {
					self.character as u8
				}
			},
			colour: self.colour,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Frame {
	pub position: Position,
	pub dimensions: Dimensions,
	frame: Vec<Vec<ColouredChar>>,
}

impl Frame {
	pub fn new(position: Position, dimensions: Dimensions) -> Result<Frame, GuiError> {
		Ok(Frame {
			position,
			dimensions,
			frame: vec![vec![ColouredChar::null(); dimensions.x]; dimensions.y],
		})
	}
	pub fn render(&self) -> Vec<Vec<ColouredChar>> {
		self.frame.clone()
	}

	pub fn render_screen_char(&self) -> Vec<Vec<ScreenChar>> {
		self.frame.clone().into_iter().map(|row| {
			row.into_iter().map(|char| {
				char.as_screen_char()
			}).collect::<Vec<ScreenChar>>()
		}).collect::<Vec<Vec<ScreenChar>>>()
	}
	pub fn position(&self) -> Position {
		self.position
	}
	pub fn dimensions(&self) -> Dimensions {
		self.dimensions
	}
	pub fn set_pos(&mut self, position: Position, char: ColouredChar) {
		self.frame[position.y][position.x] = char
	}
	pub fn render_element(&mut self, other: &Frame) {
		serial_println!("frame:\n{}",
			other.frame.iter().map(|x| {
				x.iter().map(|y| {
					y.character
				}).collect::<String>()
			}).collect::<Vec<String>>().join("\n")
		);

		for (i, row) in other.frame.iter().enumerate() {
			for (j, chr) in row.iter().enumerate() {
				self.frame[i + other.position.y][j + other.position.x] = *chr
			}
		}

		serial_println!("self:\n{}",
			self.frame.iter().map(|x| {
				x.iter().map(|y| {
					y.character
				}).collect::<String>()
			}).collect::<Vec<String>>().join("\n")
		);
	}

	pub fn render_bounds_check(&self, element: &Frame, should_panic: bool) -> Result<(), XorY> {
		use XorY::{X, Y};

		let mut res = XorY::None;

		if element.dimensions().x + element.position().x > self.dimensions.x {
			if should_panic {
				panic!(
					"Element is to large to be rendered {} {}",
					element.dimensions().x + element.position().x,
					self.dimensions.x
				)
			} else {
				res.setx();
			}
		}

		if element.dimensions().y + element.position().y > self.dimensions.y {
			if should_panic {
				panic!(
					"Element is to large to be rendered {} {}",
					element.dimensions().y + element.position().y,
					self.dimensions.y
				)
			} else {
				res.sety();
			}

		}

		if res != XorY::None {
			Err(res)
		} else {
			Ok(())
		}
	}
}

pub trait CgOutline {
	fn render_outline(&self, frame: &mut Frame);
}


pub trait CgComponent {
	fn render(&self) -> Result<Frame, GuiError>;
}














