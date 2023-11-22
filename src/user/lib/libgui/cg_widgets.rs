use alloc::{
    boxed::Box,
    string::String,
    vec,
    vec::Vec,
};
use crate::serial_println;
use super::cg_core::{
    Position, Dimensions, ColouredChar, CgComponent, CgOutline, Frame, GuiError,
};




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
            frame.set_pos(Position::new(i, 0), ColouredChar::white('─'));
            frame.set_pos(Position::new(i, frame.dimensions.y - 1), ColouredChar::white('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.set_pos(Position::new(0, i), ColouredChar::white('│'));
            frame.set_pos(Position::new(frame.dimensions.x - 1, i), ColouredChar::white('│'));
        }

        // draws the corners of the container
        frame.set_pos(Position::new(0, 0), ColouredChar::white('┌'));
        frame.set_pos(Position::new(self.dimensions.x - 1, 0), ColouredChar::white('┐'));
        frame.set_pos(Position::new(0, self.dimensions.y - 1), ColouredChar::white('└'));
        frame.set_pos(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::white('┘'));
    }
}

impl CgComponent for CgContainer {
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


pub struct CgTextBox {
    title: String,
    content: String,
    pub position: Position,
    pub dimensions: Dimensions,
    outlined: bool,
}

impl CgTextBox {
    pub fn new(title: String, content: String, position: Position, dimensions: Dimensions, outlined: bool) -> CgTextBox {
        CgTextBox { title, content, position, dimensions, outlined }
    }
}

impl CgComponent for CgTextBox {
    fn render(&self) -> Result<Frame, GuiError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        if self.outlined {
            self.render_outline(&mut result);
        }

        let title = self.title.chars();
        for (i, c) in title.enumerate() {
            if i + 2 == self.dimensions.x - 3 { // we dont want to write at the top of the text box
                result.set_pos(Position::new(i + 1, 0), ColouredChar::white('.'));
            } else if i + 2 >= self.dimensions.x - 2 {
                result.set_pos(Position::new(i + 1, 0), ColouredChar::white('.'));
                break;

            }
            result.set_pos(Position::new(i + 2, 0), ColouredChar::white(c));
        }

        let (mut x, mut y) = (1,1);

        for c in self.content.chars() {
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
                        result.set_pos(Position::new(self.dimensions.x - z, self.dimensions.y -1), ColouredChar::white('.'));
                    })
                }
                break;
            }

            result.set_pos(Position::new(x, y), ColouredChar::white(c));
            x += 1;
        };

        Ok(result)
    }
}

impl CgOutline for CgTextBox {
    fn render_outline(&self, frame: &mut Frame) {
        // draws the sides of the container
        for i in 0..frame.dimensions.x {
            frame.set_pos(Position::new(i, 0), ColouredChar::white('─'));
            frame.set_pos(Position::new(i, frame.dimensions.y - 1), ColouredChar::white('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.set_pos(Position::new(0, i), ColouredChar::white('│'));
            frame.set_pos(Position::new(frame.dimensions.x - 1, i), ColouredChar::white('│'));
        }

        // draws the corners of the container
        frame.set_pos(Position::new(0, 0), ColouredChar::white('┌'));
        frame.set_pos(Position::new(self.dimensions.x - 1, 0), ColouredChar::white('┐'));
        frame.set_pos(Position::new(0, self.dimensions.y - 1), ColouredChar::white('└'));
        frame.set_pos(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::white('┘'));
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
    fn render(&self) -> Result<Frame, GuiError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        let shortened_string = self.content.chars().take(self.dimensions.x).collect::<String>();

        for (i, c) in shortened_string.chars().enumerate() {
            result.set_pos(Position::new(i, 0), ColouredChar::white(c));
        };

        serial_println!("{:?}", result);

        Ok(result)
    }
}









































