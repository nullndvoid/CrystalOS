use alloc::string::String;
use alloc::vec::Vec;
use crate::std::frame::{ColouredChar, Dimensions, Frame, Position, RenderError};
use crate::user::lib::libgui::cg_core::{CgComponent, CgTextEdit};

#[derive(Debug, Clone)]
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
        let mut idx = 0;

        for c in self.prompt.chars() {
            if idx >= self.dimensions.x {
                break;
            }
            frame.write(Position::new(idx, 0), ColouredChar::new(c));
            idx += 1
        }

        idx += 1; // create a space between the prompt and the text

        if idx + self.text.len() > self.dimensions.x {
            frame.write(Position::new(idx, 0), ColouredChar::new('['));
            frame.write(Position::new(idx + 1, 0), ColouredChar::new('.'));
            frame.write(Position::new(idx + 2, 0), ColouredChar::new('.'));
            frame.write(Position::new(idx + 3, 0), ColouredChar::new('.'));
            frame.write(Position::new(idx + 4, 0), ColouredChar::new(']'));
            idx += 5
        }


        self.text.iter().rev().take(self.dimensions.x - idx).rev().for_each(|c| {
            frame.write(Position::new(idx, 0), ColouredChar::new(*c));
            idx += 1
        });

        Ok(frame)
    }
}

impl CgTextEdit for CgLineEdit {
    fn write_char(&mut self, c: char) {
        self.text.insert(self.ptr, c);
        self.ptr += 1;
    }
    fn backspace(&mut self) {
        if self.ptr > 0 {
            self.ptr -= 1;
            self.text.remove(self.ptr);
        }
    }
    fn move_cursor(&mut self, direction: bool) {
        match direction {
            true => if self.ptr < self.text.len() { self.ptr += 1; },
            false => if self.ptr > 0 { self.ptr -= 1; },
        }
    }
    fn clear(&mut self) {
        self.text.clear();
        self.ptr = 0
    }
}
