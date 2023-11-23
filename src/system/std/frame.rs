use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use crate::kernel::render::{BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, RENDERER, ScreenChar};
use crate::{println, serial_println};
use spin::Mutex;
use crate::std::io::{Color, Screen};

/// TODO: get a working implementation for CLI apps
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
///
/// nothing will appear on the screen until the frame is actually rendered by
/// the render_frame method on the renderer

pub use crate::system::kernel::render::{special_char, RenderError};




#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColouredChar {
    pub character: char,
    pub colour: ColorCode,
}

impl ColouredChar {
    pub fn coloured(character: char, colour: ColorCode) -> ColouredChar {
        ColouredChar { character, colour }
    }
    pub fn new(character: char) -> ColouredChar {
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

pub type Dimensions = Position;


#[derive(Clone, Debug)]
pub struct Frame {
    pub position: Position,
    pub dimensions: Dimensions,
    pub frame: Vec<Vec<ColouredChar>>,
}


impl Frame {
    pub fn new(position: Position, dimensions: Dimensions) -> Result<Frame, RenderError> {
        Ok(Frame {
            position,
            dimensions,
            frame: vec![vec![ColouredChar::null(); dimensions.x]; dimensions.y],
        })
    }

    pub fn from_str(elemstr: String) -> Self {
        let mut element = Frame { frame: Vec::<Vec<ColouredChar>>::new(), dimensions: Dimensions::new(0, 0), position: Position::new(0, 0) };

        for line in elemstr.split("\n") {
            element.frame.push(
                line
                    .chars()
                    .map(|c| ColouredChar::new(c))
                    .collect::<Vec<ColouredChar>>()
            );
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.x as usize {
                element.dimensions.x = n;
            }
        }
        element
    }

    pub fn render(&self) -> Vec<Vec<ColouredChar>> {
        self.frame.clone()
    }

    pub fn render_to_screen(&self) -> Result<(), RenderError> {
        let mut frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                //println!("{} {} {}", i, j, col);
                frame[i + self.position.y as usize][j + self.position.x as usize] = col.as_screen_char();
            };
        }
        RENDERER.lock().render_frame(frame);
        Ok(())
    }
    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
    pub fn write_pos(&mut self, position: Position, char: ColouredChar) {
        self.frame[position.y][position.x] = char
    }
    pub fn render_element(&mut self, other: &Frame) {
        for (i, row) in other.frame.iter().enumerate() {
            for (j, chr) in row.iter().enumerate() {
                self.frame[i + other.position.y][j + other.position.x] = *chr
            }
        }
    }

    pub fn render_bounds_check(&self, element: &Frame, should_panic: bool) -> Result<(), RenderError> {

        let (mut x, mut y) = (false, false);

        if element.dimensions().x + element.position().x > self.dimensions.x {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().x + element.position().x,
                self.dimensions.x
            )} else {
                x = true;
            }
        }

        if element.dimensions().y + element.position().y > self.dimensions.y {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().y + element.position().y,
                self.dimensions.y
            )} else {
                y = true;
            }
        }

        if x || y {
            Err(RenderError::OutOfBounds(x, y))
        } else {
            Ok(())
        }
    }
}

impl core::ops::Index<usize> for Frame {
    type Output = Vec<ColouredChar>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.frame[index]
    }
}

impl core::ops::IndexMut<usize> for Frame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.frame[index]
    }
}
































