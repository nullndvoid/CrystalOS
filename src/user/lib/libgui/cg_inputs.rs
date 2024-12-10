use crate::std::application::Exit;
use crate::std::io::{KeyStroke, Stdin};
use crate::std::render::{ColouredChar, Dimensions, Frame, Position, RenderError};
use crate::user::lib::libgui::cg_core::{CgComponent, CgTextEdit, CgTextInput, Widget};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use async_trait::async_trait;
use core::any::Any;

#[derive(Debug, Clone)]
pub struct CgLineEdit {
    pub position: Position<usize>,
    pub dimensions: Dimensions<usize>,
    pub prompt: String,
    pub text: Vec<char>,
    pub ptr: usize, // cursor position
}

impl CgLineEdit {
    pub fn new(position: Position<usize>, width: usize, prompt: String) -> CgLineEdit {
        CgLineEdit {
            position,
            dimensions: Dimensions::new(width, 1),
            prompt,
            text: Vec::new(),
            ptr: 0,
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
            frame
                .write(Position::new(idx, 0), ColouredChar::new(c))
                .unwrap();
            idx += 1
        }

        idx += 1; // create a space between the prompt and the text

        if idx + self.text.len() > self.dimensions.x {
            frame
                .write(Position::new(idx, 0), ColouredChar::new('['))
                .unwrap();
            frame
                .write(Position::new(idx + 1, 0), ColouredChar::new('.'))
                .unwrap();
            frame
                .write(Position::new(idx + 2, 0), ColouredChar::new('.'))
                .unwrap();
            frame
                .write(Position::new(idx + 3, 0), ColouredChar::new('.'))
                .unwrap();
            frame
                .write(Position::new(idx + 4, 0), ColouredChar::new(']'))
                .unwrap();
            idx += 5
        }

        self.text
            .iter()
            .rev()
            .take(self.dimensions.x - idx)
            .rev()
            .for_each(|c| {
                frame
                    .write(Position::new(idx, 0), ColouredChar::new(*c))
                    .unwrap();
                idx += 1
            });

        Ok(frame)
    }
    fn as_any(&self) -> &dyn Any {
        self
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
            true => {
                if self.ptr < self.text.len() {
                    self.ptr += 1;
                }
            }
            false => {
                if self.ptr > 0 {
                    self.ptr -= 1;
                }
            }
        }
    }
    fn clear(&mut self) {
        self.text.clear();
        self.ptr = 0
    }
}

#[async_trait]
impl CgTextInput for CgLineEdit {
    async fn input(
        &mut self,
        break_condition: fn(KeyStroke) -> (KeyStroke, Exit),
        id: &Widget,
        app: &Widget,
    ) -> Result<(String, bool), RenderError> {
        loop {
            match break_condition(Stdin::keystroke().await) {
                (KeyStroke::Char('\n'), Exit::None) => {
                    let res = self.text.iter().collect();
                    self.clear();
                    id.update(self.clone());
                    match app.render() {
                        Ok(frame) => frame.write_to_screen()?,
                        Err(e) => return Err(e),
                    }
                    return Ok((res, false));
                }
                (c, Exit::None) => {
                    match c {
                        KeyStroke::Char('\x08') => self.backspace(),
                        KeyStroke::Backspace => self.backspace(),
                        KeyStroke::Char(c) => self.write_char(c),
                        KeyStroke::Left => self.move_cursor(false),
                        KeyStroke::Right => self.move_cursor(true),
                        _ => (),
                    }

                    id.update(self.clone());
                    match app.render() {
                        Ok(frame) => frame.write_to_screen()?,
                        Err(e) => return Err(e),
                    }
                }
                (_, Exit::Exit) => return Ok((String::new(), true)),
                _ => (),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CgBoxEdit {
    pub position: Position<usize>,
    pub dimensions: Dimensions<usize>,
    pub prompt: String,
    pub text: Vec<char>,
    pub ptr: Position<usize>,
}

impl CgBoxEdit {
    pub fn new(
        position: Position<usize>,
        dimensions: Dimensions<usize>,
        prompt: String,
    ) -> CgBoxEdit {
        CgBoxEdit {
            position,
            dimensions,
            prompt,
            text: Vec::new(),
            ptr: Position::new(0, 0),
        }
    }
}
