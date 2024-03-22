use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::system::kernel::render::{RENDERER, ScreenChar};
use crate::std::io::Color;

/// TODO: get a working implementation for CLI apps
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
///
/// nothing will appear on the screen until the frame is actually rendered by
/// the write_to_screen() method on the renderer

pub use crate::system::kernel::render::{
    special_char,
    RenderError,
    ColorCode,
    BUFFER_WIDTH,
    BUFFER_HEIGHT
};


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
                line.chars()
                    .map(|c| ColouredChar::new(c))
                    .collect::<Vec<ColouredChar>>()
            );
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.x {
                element.dimensions.x = n;
            }
        }
        element
    }

    pub fn render(&self) -> Vec<Vec<ColouredChar>> {
        self.frame.clone()
    }

    pub fn write_to_screen(&self) -> Result<(), RenderError> {
        let mut frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                frame[i + self.position.y][j + self.position.x] = col.as_screen_char();
            };
        }
        RENDERER.lock().render_frame(frame);
        Ok(())
    }
    pub fn get_position(&self) -> Position {
        self.position
    }
    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
    pub fn write(&mut self, position: Position, char: ColouredChar) -> Result<(), RenderError> {
        if position.x >= self.dimensions.x || position.y >= self.dimensions.y {
            return Err(RenderError::OutOfBounds(
                position.x >= self.dimensions.x,
                position.y >= self.dimensions.y,
            ));
        }
        self.frame[position.y][position.x] = char;
        Ok(())
    }
    pub fn place_child_element(&mut self, other: &Frame) {
        for (i, row) in other.frame.iter().enumerate() {
            for (j, chr) in row.iter().enumerate() {
                self.frame[i + other.position.y][j + other.position.x] = *chr
            }
        }
    }

    pub fn render_bounds_check(&self, element: &Frame, should_panic: bool) -> Result<(), RenderError> {

        let (mut x, mut y) = (false, false);

        if element.dimensions().x + element.get_position().x > self.dimensions.x {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().x + element.get_position().x,
                self.dimensions.x
            )} else {
                x = true;
            }
        }

        if element.dimensions().y + element.get_position().y > self.dimensions.y {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().y + element.get_position().y,
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
































