use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::kernel::render::ScreenChar;
use crate::printerr;
use crate::user::lib::gui_v2::widgets::XorY::Both;

#[derive(Copy, Clone)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

#[derive(Copy, Clone)]
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


pub enum GuiError {
	OutOfBounds(XorY)
}

pub type Dimensions = Position;
pub type ColouredChar = ScreenChar;

#[derive(Copy, Clone)]
pub struct Frame {
	position: Position,
	dimensions: Dimensions,
	frame: Vec<Vec<ColouredChar>>,
}

impl Frame {
	pub fn new(position: Position, dimensions: Dimensions) -> Frame {
		Frame {
			position,
			dimensions,
			frame: vec![vec![ScreenChar::null(); dimensions.x]; dimensions.y],
		}
	}
	pub fn render(&self) -> Vec<Vec<ColouredChar>> {
		self.frame.clone()
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
		for (i, row) in other.frame.iter().enumerate() {
			for (j, chr) in row.iter().enumerate() {
				self.frame[i + self.position.y][j + self.position.x] = *chr
			}
		}
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
					.dimensions().y + element.position().y,
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


pub trait GuiComponent {
	fn render(&self) -> Frame;
}


pub struct Container {
	pub frame: Vec<Vec<ColouredChar>>,
	pub elements: Vec<dyn GuiComponent>,
	pub position: Position,
	pub dimensions: Dimensions,
}

impl GuiComponent for Container {
	fn render(&self) -> Result<Frame, GuiError> {

		let mut frame = Frame::new(self.position, self.dimensions);

		for element in &self.elements {
			match frame.render_bounds_check(element, true) { // TODO: this needs to be set to false for production
				Ok(()) => {
					let r = (*element).render();
					frame.render_element(&r);
				}
				Err(e) => {
					return Err(GuiError::OutOfBounds(e));
				}
			}
		}
		
		Ok(frame)
	}
}










