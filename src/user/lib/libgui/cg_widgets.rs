use alloc::{boxed::Box, format, string::String, vec, vec::Vec};
use alloc::fmt::format;
use alloc::string::ToString;
use core::any::Any;
use core::cmp::{max, min};
use hashbrown::HashMap;
use crate::serial_println;
use super::cg_core::{CgComponent, CgOutline, Widget};
use crate::std::frame::{ColouredChar, Dimensions, Position, Frame, RenderError, ColorCode};
use crate::std::io::Color;

#[derive(Debug, Clone)]
pub struct CgContainer {
    pub elements: HashMap<&'static str, Widget>,
    pub position: Position,
    pub dimensions: Dimensions,
    pub outlined: bool,
}

impl CgContainer {
    pub fn new(position: Position, dimensions: Dimensions, outlined: bool) -> CgContainer {
        CgContainer {
            elements: HashMap::new(),
            position,
            dimensions,
            outlined,
        }
    }
    pub fn insert(&mut self, name: &'static str, element: Widget) {
        self.elements.insert(name,element);
    }
    pub fn fetch(&self, name: &'static str) -> Option<&Widget> {
        self.elements.get(name)
    }
}

impl CgOutline for CgContainer {
    fn render_outline(&self, frame: &mut Frame) {
        // draws the sides of the container
        for i in 0..frame.dimensions.x {
            frame.write(Position::new(i, 0), ColouredChar::new('─'));
            frame.write(Position::new(i, frame.dimensions.y - 1), ColouredChar::new('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.write(Position::new(0, i), ColouredChar::new('│'));
            frame.write(Position::new(frame.dimensions.x - 1, i), ColouredChar::new('│'));
        }

        // draws the corners of the container
        frame.write(Position::new(0, 0), ColouredChar::new('┌'));
        frame.write(Position::new(self.dimensions.x - 1, 0), ColouredChar::new('┐'));
        frame.write(Position::new(0, self.dimensions.y - 1), ColouredChar::new('└'));
        frame.write(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::new('┘'));
    }
}

impl CgComponent for CgContainer {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;
        for widget in &self.elements {
            let frame = widget.1.render()?;
            match result.render_bounds_check(&frame, true) { // TODO: this needs to be set to false for production
                Ok(()) => result.place_child_element(&frame),
                Err(e) => return Err(e),
            }
        }

        if self.outlined {
            self.render_outline(&mut result);
        }

        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct CgTextBox {
    title: String,
    pub content: String,
    pub position: Position,
    pub dimensions: Dimensions,
    outlined: bool,
    wrap_words: bool // if false then will not wrap until the end of a word if possible
}

impl CgTextBox {
    pub fn new(title: String, content: String, position: Position, dimensions: Dimensions, outlined: bool) -> CgTextBox {
        CgTextBox { title, content, position, dimensions, outlined, wrap_words: true }
    }

    fn render_title(&self, frame: &mut Frame) {
        let title = self.title.chars();
        for (i, c) in title.enumerate() {
            if i + 2 == self.dimensions.x - 3 { // we dont want to write at the top of the text box
                frame.write(Position::new(i + 1, 0), ColouredChar::new('.'));
            } else if i + 2 >= self.dimensions.x - 2 {
                frame.write(Position::new(i + 1, 0), ColouredChar::new('.'));
                break;
            }
            frame.write(Position::new(i + 2, 0), ColouredChar::new(c));
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
                            result.write(Position::new(self.dimensions.x - z, self.dimensions.y - 1), ColouredChar::new('.'));
                        })
                    }
                    break;
                }

                result.write(Position::new(x, y), ColouredChar::new(c));
                x += 1;
            };
        }

        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl CgOutline for CgTextBox {
    fn render_outline(&self, frame: &mut Frame) {
        // draws the sides of the container
        for i in 0..frame.dimensions.x {
            frame.write(Position::new(i, 0), ColouredChar::new('─'));
            frame.write(Position::new(i, frame.dimensions.y - 1), ColouredChar::new('─'));
        }

        // draws the top and bottom of the container
        for i in 0..frame.dimensions.y {
            frame.write(Position::new(0, i), ColouredChar::new('│'));
            frame.write(Position::new(frame.dimensions.x - 1, i), ColouredChar::new('│'));
        }

        // draws the corners of the container
        frame.write(Position::new(0, 0), ColouredChar::new('┌'));
        frame.write(Position::new(self.dimensions.x - 1, 0), ColouredChar::new('┐'));
        frame.write(Position::new(0, self.dimensions.y - 1), ColouredChar::new('└'));
        frame.write(Position::new(self.dimensions.x - 1, self.dimensions.y - 1), ColouredChar::new('┘'));
    }
}




#[derive(Debug, Clone)]
pub struct CgLabel {
    content: String,
    position: Position,
    dimensions: Dimensions,
    centered: bool,
}

impl CgLabel {
    pub fn new(content: String, position: Position, width: usize, centered: bool) -> CgLabel {
        CgLabel {
            content,
            position: Position::new(position.x, position.y),
            dimensions: Dimensions::new(width, 1),
            centered,
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}

impl CgComponent for CgLabel {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        let shortened_string = self.content.chars().take(self.dimensions.x).collect::<String>();

        let left = if self.centered && self.content.len() < self.dimensions.x {
            let difference = self.dimensions.x - self.content.len();
            difference / 2
        } else {
            0
        };


        for (i, c) in shortened_string.chars().enumerate() {
            if i + left >= self.dimensions.x {
                (0..3).for_each(|z| {
                    result.write(Position::new(self.dimensions.x - z + left, self.dimensions.y - 1), ColouredChar::new('.'));
                });
                break;
            }
            result.write(Position::new(i + left, 0), ColouredChar::new(c));
        };
        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
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
    fn text(&self) -> &str {
        &self.content
    }
    fn set_text(&mut self, content: String) {
        self.content = content;
    }
}

impl CgComponent for CgIndicatorWidget {
    fn render(&self) -> Result<Frame, RenderError> {
        if !self.visible {
            return Ok(Frame::new(Position::new(0, 0), Dimensions::new(0, 0))?);
        }
        let mut result = Frame::new(Position::new(0, 0), Dimensions::new(min(self.max_width, self.content.len()), 1))?;

        let shortened_string = self.content.chars().take(self.max_width).collect::<String>();
        for (i, c) in shortened_string.chars().enumerate() {
            result.write(Position::new(i, 0), ColouredChar::coloured(c, self.colour));
        };

        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct CgIndicatorBar {
    pub fields: Vec<CgIndicatorWidget>,
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
                Ok(()) => result.place_child_element(&frame),
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct CgStatusBar {
    position: Position,
    dimensions: Dimensions,

    window_title: CgIndicatorWidget,
    screen_mode: CgIndicatorWidget,
}

impl CgComponent for CgStatusBar {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(self.position, self.dimensions)?;

        (0..80).for_each(|x| frame[0][x] = ColouredChar::coloured(' ', ColorCode::new(Color::Black, Color::DarkGray)));

        // render window title centred
        let mut window_title = self.window_title.render()?;
        let width = window_title.dimensions().x;
        window_title.set_position(Position::new((self.dimensions.x - width) / 2, 0));

        // render screen mode right
        let mut screen_mode = self.screen_mode.render()?;
        let width = screen_mode.dimensions().x;
        screen_mode.set_position(Position::new(self.dimensions.x - width, 0));

        frame.place_child_element(&window_title);
        frame.place_child_element(&screen_mode);

        Ok(frame)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CgStatusBar {
    pub fn new(position: Position, dimensions: Dimensions) -> CgStatusBar {
        let mut widget = CgStatusBar {
            position,
            dimensions,
            window_title: CgIndicatorWidget::new("feature test".to_string(), 20),
            screen_mode: CgIndicatorWidget::new("Application".to_string(), 20),
        };
        widget.window_title.set_colour(ColorCode::new(Color::Cyan, Color::DarkGray));
        widget.screen_mode.set_colour(ColorCode::new(Color::Yellow, Color::DarkGray));
        widget
    }

    pub fn set_window_title(&mut self, title: String) {
        self.window_title.set_text(title);
    }

    pub fn set_screen_mode(&mut self, mode: String) {
        self.screen_mode.set_text(mode);
    }
}
































