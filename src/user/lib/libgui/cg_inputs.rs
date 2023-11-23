use alloc::string::String;
use alloc::vec::Vec;
use crate::std::frame::{ColouredChar, Dimensions, Frame, Position, RenderError};
use crate::user::lib::libgui::cg_core::{CgComponent, CgTextEdit};

pub struct CgLineEdit {
    pub position: Position,
    pub dimensions: Dimensions,
    pub prompt: String,
    pub text: Vec<char>,
    pub ptr: usize, // cursor position
}

impl CgLineEdit {
    pub fn new(position: Position, width: usize, prompt: String) -> CgLineEdit {
        CgLineEdit {
            position,
            dimensions: Dimensions::new(width, 1),
            prompt: prompt,
            text: Vec::new(),
            ptr: 0
        }
    }
}

impl CgComponent for CgLineEdit {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(self.position, self.dimensions)?;

        let mut ptr = 0;

        for c in self.prompt.chars() {
            if ptr >= self.dimensions.x {
                break;
            }
            frame.write(Position::new(ptr, 0), ColouredChar::new(c));
            ptr += 1
        }

        ptr += 1; // create a space between the prompt and the text

        for c in self.text.iter() {
            if ptr >= self.dimensions.x {
                break;
            }
            frame.write(Position::new(ptr, 0), ColouredChar::new(*c));
            ptr += 1
        }

        Ok(frame)
    }
}

