use alloc::{boxed::Box, format, string::String, vec::Vec};
use alloc::string::ToString;
use core::any::Any;
use core::cmp::{max, min};
use async_trait::async_trait;
use hashbrown::HashMap;
use crate::std::application::Exit;
use super::cg_core::{CgComponent, CgKeyboardCapture, Widget};
use super::cg_utils::render_outline;
use crate::std::frame::{ColouredChar, Dimensions, Position, Frame, RenderError, ColorCode, BUFFER_WIDTH, BUFFER_HEIGHT};
use crate::std::io::{Color, KeyStroke, Stdin};

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
            render_outline(&mut result, self.dimensions.clone())?;
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

    fn render_title(&self, frame: &mut Frame) -> Result<(), RenderError>{
        let title = self.title.chars();
        for (i, c) in title.enumerate() {
            if i + 2 == self.dimensions.x - 3 { // we don't want to write at the top of the text box
                frame.write(Position::new(i + 1, 0), ColouredChar::new('.'))?;
            } else if i + 2 >= self.dimensions.x - 2 {
                frame.write(Position::new(i + 1, 0), ColouredChar::new('.'))?;
                break;
            }
            frame.write(Position::new(i + 2, 0), ColouredChar::new(c))?;
        };
        Ok(())
    }
    pub fn wrap_words(&mut self, wrap: bool) {
        self.wrap_words = wrap;
    }
}

impl CgComponent for CgTextBox {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut result = Frame::new(self.position, self.dimensions)?;

        if self.outlined {
            render_outline(&mut result, self.dimensions.clone())?;
        }

        self.render_title(&mut result)?;

        let (mut x, mut y) = (1, 1);

        for word in self.content.split(' ') {
            if self.wrap_words {
                if word.len() + 1 > 1 + self.dimensions.x - 2 - x {
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
                            result.write(Position::new(self.dimensions.x - z, self.dimensions.y - 1), ColouredChar::new('.')).unwrap();
                        })
                    }
                    break;
                }

                result.write(Position::new(x, y), ColouredChar::new(c))?;
                x += 1;
            };
        }

        Ok(result)
    }
    fn as_any(&self) -> &dyn Any {
        self
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
                    result.write(Position::new(self.dimensions.x - z + left, self.dimensions.y - 1), ColouredChar::new('.')).expect("failed to write");
                });
                break;
            }
            result.write(Position::new(i + left, 0), ColouredChar::new(c))?;
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
            result.write(Position::new(i, 0), ColouredChar::coloured(c, self.colour)).expect("failed to render indicator widget");
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

        (0..BUFFER_WIDTH).for_each(|x| frame[0][x] = ColouredChar::coloured(' ', ColorCode::new(Color::Black, Color::DarkGray)));

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

pub enum CgDialogType {
    Information,
    Confirmation,
    Selection(Vec<String>),
}

pub struct CgDialog {
    width: usize,
    title: String,
    content: String,
    accepted: bool,
    outlined: bool,
    pub dialog_class: CgDialogType,
    pub selected_idx: usize,
}

impl CgDialog {
    pub(crate) fn new(title: String, content: String, class: CgDialogType) -> CgDialog {
        CgDialog {
            width: 40,
            title,
            content,
            accepted: false,
            outlined: true,
            dialog_class: class,
            selected_idx: 0,
        }
    }
}


// TODO: make dialogs responsive.
impl CgComponent for CgDialog {
    fn render(&self) -> Result<Frame, RenderError> {

        // find the size needed for the dialog buttons
        let dialog_button_width = match &self.dialog_class {
            CgDialogType::Selection(options) => {
                options.iter().fold(0, |sum, x| sum + 5 + x.len()) // [ Option ] for each option
            },
            CgDialogType::Information => 6, // [ Ok ]
            CgDialogType::Confirmation => 22  // [ Confirm ] [ Cancel ]
        };

        // picks the largest out of the title length, dialog button length and 40 to determine the minimum width of the dialog.
        let mut width = max(max(self.title.len(), dialog_button_width), 40);

        // we set the base height to 5, assuming the content is none.
        let mut height = 5;

        // calculate required width and height of textbox based on the size of the content.
        while self.content.len() as f32 * 1.25 / width as f32 >= BUFFER_HEIGHT as f32 - 8.0 + 1.0 { // the + 1.0 accounts for decimal values being truncated down, ensuring that the max height of 25 can be reached.
            if width < BUFFER_WIDTH - 4 {
                width += 1;
            } else {
                // in the case that the text does not fit within the dialog
                // TODO: handle this properly
                return Err(RenderError::OutOfBounds(true, true));
            }
        };
        height = (self.content.len() as f32 * 1.25 / (width as f32)) as usize + 1;

        // account for borders
        width += 4;
        height += 8;
        
        // offsets to centre the dialog
        let x_offset = (BUFFER_WIDTH - width) / 2;
        let y_offset = (BUFFER_HEIGHT - height) / 2;

        // now that we know the X and Y offsets, we can start to draw the frame
        let mut frame = Frame::new(Position::new(x_offset, y_offset), Dimensions::new(width, height))?;
        if let Err(e) = render_outline(&mut frame, Dimensions::new(width, height)) {
            return Err(e);
        }

        
        // render title
        let title_offset = (width - self.title.len()) / 2;
        let title = CgLabel::new(self.title.clone(), Position::new(title_offset, 2), self.title.len(), true);
        frame.place_child_element(&title.render().unwrap());


        let (mut x, mut y) = (2, 4); // top left of the text box
        for word in self.content.split(' ') {
            if word.len() + 1 > 1 + width - 4 - x { // adding a +1 on both sides accounts for the possible negative value at the end of the line, avoiding integer underflow.
                if word.len() <= width - 4 {
                    x = 2;
                    y += 1;
                }
            }

            for c in format!("{} ", word).chars() {
                if x >= width - 3 {
                    x = 2;
                    y += 1;
                    if c == ' ' {
                        continue;
                    }
                }
                if y >= height - 4 {
                    break;
                }

                frame.write(Position::new(x, y), ColouredChar::new(c)).unwrap();
                x += 1;
            };
        }
        
        // dialog buttons
        match &self.dialog_class {
            CgDialogType::Information => {
                let button_x_offset = (width - 6) / 2;
                "[ Ok ]".chars().enumerate().for_each(|(i, c)| {
                    frame.write(Position::new(button_x_offset + i, height - 3), ColouredChar {
                        character: c,
                        colour: ColorCode::new(Color::Cyan, Color::Black),
                    }).expect("failed to write to frame, perhaps buttons were placed wrongly or width too great");
                })
            }
            CgDialogType::Confirmation => {
                let button_fmt = ["Cancel", "Confirm"]
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i == self.selected_idx {
                            format!("[{}] ", s).chars().map(|c| ColouredChar {
                                character: c,
                                colour: ColorCode::new(Color::Cyan, Color::Black),
                            }).collect::<Vec<ColouredChar>>()
                        } else {
                            format!("[{}] ", s).chars().map(|c| ColouredChar::new(c)).collect()
                        }
                    }).flatten()
                    .collect::<Vec<ColouredChar>>();

                let button_x_offset = (width - button_fmt.len()) / 2;
                button_fmt.into_iter().enumerate().for_each(|(i, c)| {
                    frame.write(Position::new(button_x_offset + i, height - 3), c).expect("failed to write to frame, perhaps buttons were placed wrongly or width too great");
                })
            },
            CgDialogType::Selection(options) => {
                let button_fmt = options
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i == self.selected_idx {
                            format!("[{}] ", s).chars().map(|c| ColouredChar {
                                character: c,
                                colour: ColorCode::new(Color::Cyan, Color::Black),
                            }).collect::<Vec<ColouredChar>>()
                        } else {
                            format!("[{}] ", s).chars().map(|c| ColouredChar::new(c)).collect()
                        }
                    }).flatten()
                    .collect::<Vec<ColouredChar>>();

                let button_x_offset = (width - button_fmt.len()) / 2;
                button_fmt.into_iter().enumerate().for_each(|(i, c)| {
                    frame.write(Position::new(button_x_offset + i, height - 3), c).expect("failed to write to frame, perhaps buttons were placed wrongly or width too great");
                })
            }
        };
        

        Ok(frame)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl CgKeyboardCapture for CgDialog {
    async fn keyboard_capture(&mut self, break_condition: fn(KeyStroke) -> (KeyStroke, Exit), app: Option<&Widget>) -> Result<(Exit, usize), RenderError> {
        loop {
            let k = break_condition(Stdin::keystroke().await);
            match k {
                (_, Exit::Exit) => { // this handles the "exit" keybind given to the function
                    return Ok((Exit::Exit, 0))
                }
                (KeyStroke::Char('\n'), _) => { // return the chosen option
                    return Ok((Exit::None, self.selected_idx))
                },
                (KeyStroke::Left, _) => {
                    if self.selected_idx > 0 {
                        self.selected_idx -= 1;
                        if let Ok(frame) = self.render() {
                            frame.write_to_screen()?;
                        }
                    }
                }
                (KeyStroke::Right, _) => {
                    match &self.dialog_class {
                        CgDialogType::Information => continue,
                        CgDialogType::Confirmation => {
                            if self.selected_idx == 0 {
                                self.selected_idx += 1;
                                if let Ok(frame) = self.render() {
                                    frame.write_to_screen()?;
                                }
                            }

                        }
                        CgDialogType::Selection(options) => {
                            if self.selected_idx < options.len() - 1 {
                                self.selected_idx += 1;
                                if let Ok(frame) = self.render() {
                                    frame.write_to_screen()?;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl CgDialog {
    pub type Type = CgDialogType;

    fn dynamic_layout(&mut self) {
    }
}
























