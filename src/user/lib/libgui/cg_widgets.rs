use alloc::{boxed::Box, format, string::String, vec, vec::Vec};
use alloc::fmt::format;
use crate::kernel::render::{ColorCode, RenderError};
use crate::serial_println;
use super::cg_core::{
    CgComponent, CgOutline
};
use crate::std::frame::{ColouredChar, Dimensions, Position, Frame};
use crate::std::io::Color;


pub struct CgContainer {
    pub elements: Vec<Box<dyn CgComponent>>,
    pub position: Position,
    pub dimensions: Dimensions,
    pub outlined: bool,
}

impl CgContainer {
    pub fn new(position: Position, dimensions: Dimensions, outlined: bool) -> CgContainer {
        CgContainer {
            elements: Vec::new(),
            position,
            dimensions,
            outlined,
        }
    }
}

impl CgOutline for CgContainer {
    fn render_outline(&self, frame: &mut Frame) {
        // draws the sides of the container
        for i in 0..frame.dimensions.x {
            frame.write_pos(Position::new(i, 0), ColouredChar::new('─'));
            frame.write_pos(Position::new(i, frame.dimensions.y - 1), ColouredChar::new('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.write_pos(Position::new(0, i), ColouredChar::new('│'));
            frame.write_pos(Position::new(frame.dimensions.x - 1, i), ColouredChar::new('│'));
        }

        // draws the corners of the container
        frame.write_pos(Position::new(0, 0), ColouredChar::new('┌'));
        frame.write_pos(Position::new(self.dimensions.x - 1, 0), ColouredChar::new('┐'));
        frame.write_pos(Position::new(0, self.dimensions.y - 1), ColouredChar::new('└'));
        frame.write_pos(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::new('┘'));
    }
}

impl CgComponent for CgContainer {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        for widget in &self.elements {
            let frame = widget.render()?;
            match result.render_bounds_check(&frame, true) { // TODO: this needs to be set to false for production
                Ok(()) => result.render_element(&frame),
                Err(e) => return Err(e),
            }
        }

        if self.outlined {
            self.render_outline(&mut result);
        }

        Ok(result)
    }
}


pub struct CgTextBox {
    title: String,
    content: String,
    pub position: Position,
    pub dimensions: Dimensions,
    outlined: bool,
    wrap_words: bool // if false then will not wrap until the end of a word if possible
}

impl CgTextBox {
    pub fn new(title: String, content: String, position: Position, dimensions: Dimensions, outlined: bool) -> CgTextBox {
        CgTextBox { title, content, position, dimensions, outlined, wrap_words: false }
    }

    fn render_title(&self, frame: &mut Frame) {
        let title = self.title.chars();
        for (i, c) in title.enumerate() {
            if i + 2 == self.dimensions.x - 3 { // we dont want to write at the top of the text box
                frame.write_pos(Position::new(i + 1, 0), ColouredChar::new('.'));
            } else if i + 2 >= self.dimensions.x - 2 {
                frame.write_pos(Position::new(i + 1, 0), ColouredChar::new('.'));
                break;
            }
            frame.write_pos(Position::new(i + 2, 0), ColouredChar::new(c));
        }
    }
    pub fn wrap_words(&mut self, wrap: bool) {
        self.wrap_words = wrap;
    }
}

impl CgComponent for CgTextBox {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        if self.outlined {
            self.render_outline(&mut result);
        }

        self.render_title(&mut result);

        let (mut x, mut y) = (1, 1);

        for word in self.content.split(' ') {
            if self.wrap_words {
                if word.len() > self.dimensions.x - 2 - x {
                    if word.len() <= self.dimensions.x - 2 {
                        x = 1;
                        y += 1;
                    }
                }
            }

            for c in format!("{} ", word).chars() {
                if x == self.dimensions.x - 1 {
                    x = 1;
                    y += 1;
                    if c == ' ' {
                        continue;
                    }
                }
                if y == self.dimensions.y - 1 {
                    if c != ' ' {
                        (2..5).for_each(|z| {
                            result.write_pos(Position::new(self.dimensions.x - z, self.dimensions.y - 1), ColouredChar::new('.'));
                        })
                    }
                    break;
                }

                result.write_pos(Position::new(x, y), ColouredChar::new(c));
                x += 1;
            };
        }

        Ok(result)
    }

}


impl CgOutline for CgTextBox {
    fn render_outline(&self, frame: &mut Frame) {
        // draws the sides of the container
        for i in 0..frame.dimensions.x {
            frame.write_pos(Position::new(i, 0), ColouredChar::new('─'));
            frame.write_pos(Position::new(i, frame.dimensions.y - 1), ColouredChar::new('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.write_pos(Position::new(0, i), ColouredChar::new('│'));
            frame.write_pos(Position::new(frame.dimensions.x - 1, i), ColouredChar::new('│'));
        }

        // draws the corners of the container
        frame.write_pos(Position::new(0, 0), ColouredChar::new('┌'));
        frame.write_pos(Position::new(self.dimensions.x - 1, 0), ColouredChar::new('┐'));
        frame.write_pos(Position::new(0, self.dimensions.y - 1), ColouredChar::new('└'));
        frame.write_pos(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::new('┘'));
    }
}





pub struct CgLabel {
    content: String,
    position: Position,
    dimensions: Dimensions,
}

impl CgLabel {
    pub fn new(content: String, position: Position, width: usize) -> CgLabel {
        CgLabel {
            content,
            position: Position::new(position.x, position.y),
            dimensions: Dimensions::new(width, 1),
        }
    }
}

impl CgComponent for CgLabel {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        let shortened_string = self.content.chars().take(self.dimensions.x).collect::<String>();

        for (i, c) in shortened_string.chars().enumerate() {
            result.write_pos(Position::new(i, 0), ColouredChar::new(c));
        };

        serial_println!("{:?}", result);

        Ok(result)
    }
}

pub struct CgIndicatorWidget {
    content: String,
    colour: ColorCode,
    visible: bool,
    max_width: usize,
}
impl CgIndicatorWidget {
    pub fn new(content: String, max_width: usize) -> CgIndicatorWidget {
        CgIndicatorWidget {
            content,
            visible: true,
            colour: ColorCode::new(Color::White, Color::Black),
            max_width
        }
    }

    pub fn set_colour(&mut self, colour: ColorCode) {
        self.colour = colour;
    }
    fn visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    fn len(&self) -> usize {
        self.max_width
    }
}

impl CgComponent for CgIndicatorWidget {
    fn render(&self) -> Result<Frame, RenderError> {
        if !self.visible {
            return Ok(Frame::new(Position::new(0, 0), Dimensions::new(0, 0))?);
        }
        let mut result = Frame::new(Position::new(0, 0), Dimensions::new(self.max_width, 1))?;

        let shortened_string = self.content.chars().take(self.max_width).collect::<String>();
        for (i, c) in shortened_string.chars().enumerate() {
            result.write_pos(Position::new(i, 0), ColouredChar::coloured(c, self.colour));
        };

        Ok(result)
    }
}

pub struct CgIndicatorBar {
    pub(crate) fields: Vec<CgIndicatorWidget>,
    position: Position,
    dimensions: Dimensions,
}

impl CgIndicatorBar {
    pub fn new(position: Position, width: usize) -> CgIndicatorBar {
        CgIndicatorBar {
            fields: Vec::new(),
            position: Position::new(position.x, position.y),
            dimensions: Dimensions::new(width, 1),
        }
    }

    fn add_field(&mut self, field: CgIndicatorWidget) {
        self.fields.push(field);
    }
}

impl CgComponent for CgIndicatorBar {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        let mut width_idx = 0;

        for widget in &self.fields {
            let mut frame = widget.render()?;
            frame.set_position(Position::new(width_idx, 0));
            width_idx += widget.len();

            match result.render_bounds_check(&frame, true) {
                Ok(()) => result.render_element(&frame),
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }
}




































