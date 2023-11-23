use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::slice::from_mut;
use crate::kernel::render::{ColorCode, RenderError, ScreenChar};
use crate::{printerr, serial_println};
use crate::std::frame::{ColouredChar, Dimensions, Position, special_char, Frame};



pub trait CgOutline {
	fn render_outline(&self, frame: &mut Frame);
}

pub trait CgComponent {
	fn render(&self) -> Result<Frame, RenderError>;
}














