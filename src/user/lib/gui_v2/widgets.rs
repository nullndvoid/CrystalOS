use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::slice::from_mut;
use crate::kernel::render::ScreenChar;
use crate::printerr;
use crate::user::lib::gui_v2::widgets::XorY::Both;

#[derive(Copy, Clone)]
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
pub type ColouredChar = ScreenChar;

#[derive(Clone)]
pub struct Frame {
	position: Position,
	dimensions: Dimensions,
	frame: Vec<Vec<ColouredChar>>,
}

impl Frame {
	pub fn new(position: Position, dimensions: Dimensions) -> Result<Frame, GuiError> {
		Ok(Frame {
			position,
			dimensions,
			frame: vec![vec![ScreenChar::null(); dimensions.x]; dimensions.y],
		})
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


pub trait GuiComponent {
	fn render(&self) -> Result<Frame, GuiError>;
}


pub struct Container {
	pub frame: Vec<Vec<ColouredChar>>,
	pub elements: Vec<Box<dyn GuiComponent>>,
	pub position: Position,
	pub dimensions: Dimensions,
	pub outlined: bool,
}

impl Container {

	pub fn new(position: Position, dimensions: Dimensions, outlined: bool) -> Container {
		Container {
			frame: vec![vec![ScreenChar::null(); dimensions.x]; dimensions.y],
			elements: Vec::new(),
			position,
			dimensions,
			outlined,
		}
	}

	fn render_outline(&self, frame: &mut Frame) {
		// draws the sides of the container
		for i in 0..frame.dimensions.x {
			frame.set_pos(Position::new(i, 0), ColouredChar::white('│' as u8));
			frame.set_pos(Position::new(i, frame.dimensions.y - 1), ColouredChar::white('│' as u8));
		}

		// draws the top and bottom of the container
		for i in 0..frame.dimensions.y {
			frame.set_pos(Position::new(0, i), ColouredChar::white('─' as u8));
			frame.set_pos(Position::new(frame.dimensions.x - 1, i), ColouredChar::white('─' as u8));
		}

		// draws the corners of the container
		frame.set_pos(Position::new(0, 0), ColouredChar::white('┌' as u8));
		frame.set_pos(Position::new(self.dimensions.x - 1, 0), ColouredChar::white('┐' as u8));
		frame.set_pos(Position::new(0, self.dimensions.y - 1), ColouredChar::white('└' as u8));
		frame.set_pos(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::white('┘' as u8));
	}
}


impl GuiComponent for Container {
	fn render(&self) -> Result<Frame, GuiError> {
		let mut result = Frame::new(self.position, self.dimensions)?;

		for widget in &self.elements {
			let frame = widget.render()?;
			match result.render_bounds_check(&frame, true) { // TODO: this needs to be set to false for production
				Ok(()) => result.render_element(&frame),
				Err(e) => return Err(GuiError::OutOfBounds(e)),
			}
		}

		if self.outlined {
			self.render_outline(&mut result);
		}

		Ok(result)
	}
}










