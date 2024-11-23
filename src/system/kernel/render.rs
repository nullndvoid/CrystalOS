use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use alloc::vec;
use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub character: u8,
    pub colour: ColorCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    OutOfBounds(bool, bool), // (bool, bool) refers to x and y respectively
    TooSmall,
    InvalidCharacter,
    InvalidColour,
    InvalidRenderMode,
}

impl ScreenChar {
    pub fn null() -> ScreenChar {
        ScreenChar {
            character: 0u8,
            colour: ColorCode::new(Color::White, Color::Black),
        }
    }
    pub fn white(mut character: u8) -> ScreenChar {
        if let Some(c) = special_char(character as char) {
            character = c;
        }
        ScreenChar {
            character,
            colour: ColorCode::new(Color::White, Color::Black),
        }
    }
    pub fn new(mut character: u8, colour: ColorCode) -> ScreenChar {
        if let Some(c) = special_char(character as char) {
           character = c
        }
        ScreenChar {
            character,
            colour,
        }
    }
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VGAOutput {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Renderer {
    col_pos: usize,
    screen_ref: &'static mut VGAOutput, // this should not be accessed unless the screen is rendering a new frame
    term_buffer: Vec<[ScreenChar; BUFFER_WIDTH]>, // this is the standard terminal output view
    app_buffer: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT], // this is where applications render their frames to
    application_mode: bool, // if false: term mode; if true: app mode
    temp_colour: Option<ColorCode>,
}

lazy_static! {
    pub static ref RENDERER: Mutex<Renderer> = Mutex::new(Renderer {
        col_pos: 0,
        screen_ref: unsafe { &mut *(0xb8000 as *mut VGAOutput) },
        term_buffer: vec![[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT],
        app_buffer: [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT],
        application_mode: false,
        temp_colour: None,
    });
}

impl Renderer {
    // EXTERNAL API : for use by standard library and other parts of the kernel
    pub fn render_frame(&mut self, frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]) { // renders the given frame to the app buffer
        let mut processed_frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];

        for (i, row) in frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                processed_frame[i][j] = match special_char(col.character as char) {
                    Some(c) => ScreenChar::new(c as u8, col.colour),
                    None => *col,
                };
            }
        }

        self.app_buffer = processed_frame;
        self.internal_render();
    }

    pub fn terminal_mode_force(&mut self) { // THIS SHOULD ONLY BE USED WHEN THE KERNEL PANICS
                                            // TODO: find a way to make this function kernel only
        self.application_mode = false;
        self.internal_render();
    }

    pub fn application_mode(&mut self) {
        self.application_mode = true;
        self.internal_render();
    }

    pub fn terminal_mode(&mut self) {
        self.application_mode = false;
        self.internal_render();
    }

    pub fn mode_is_app(&self) -> bool {
        self.application_mode
    }

    pub fn write_char(&mut self, ch: u8, col: Option<ColorCode>) { // default colour if no colour is selected for character
        if self.application_mode { return; };
        self.write_byte(ch, col);
        self.internal_render();
    }

    pub fn write_string(&mut self, string: &str, col: Option<ColorCode>) {
        if self.application_mode { return; };
        for ch in string.chars() {
            match special_char(ch) {
                Some(c) => self.write_byte(c, col),
                None => match ch as u8 {
                    0x20..=0xff | b'\n' => self.write_byte(ch as u8, col),
                    _ => self.write_byte(0xfe, col),
                }
            }
        }
        self.internal_render();
    }

    pub fn backspace(&mut self) -> Result<(), RenderError> {
        if self.application_mode { return Ok(()); };

        loop {
            if self.internal_backspace()? {
                break;
            }
        }

        self.internal_render();
        Ok(())
    }

    pub fn clear(&mut self) { // clears the screen and all scroll-back
        if self.application_mode { return; };

        self.term_buffer = vec![[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];
        self.internal_render();
    }

    pub fn set_colour(&mut self, cols: ColorCode) {
        self.temp_colour = Some(cols);
    }
    pub fn reset_colour(&mut self) {
        self.temp_colour = None;
    }

    pub fn cursor_position(&mut self, x: u8, y: u8) -> Result<(), RenderError> {
        // check that x and y are within bounds
        if x >= 80 || y >= 25 {
            return Err(RenderError::OutOfBounds(
                x >= 80,
                y >= 25
            ))
        }
        self.internal_set_cursor_position(x, y);
        Ok(())
    }

    // INTERNAL API ONLY

    fn internal_set_cursor_position(&mut self, x: u8, y: u8) {
        use x86_64::instructions::port::Port;
        let cursor_position: u16 = (y as u16) * 80 + (x as u16);

        unsafe {// Write the high byte of the cursor position to register 14
            let mut control_port = Port::<u8>::new(0x3D4);
            control_port.write(14);
            // Write the high byte of the cursor position to register 15
            let mut data_port = Port::<u8>::new(0x3D5);
            data_port.write((cursor_position >> 8) as u8);
            // Write the low byte of the cursor position to register 14
            control_port.write(15);
            // Write the low byte of the cursor position to register 15
            data_port.write((cursor_position & 0xFF) as u8);
        }
    }

    fn internal_backspace(&mut self) -> Result<bool, RenderError> {
        let mut should_break = false;

        if self.col_pos == 0 {
            self.internal_lastline();
        }
        self.col_pos -= 1;
        let col = self.col_pos;

        let buff_len = self.term_buffer.len();

        if self.term_buffer[buff_len - 1][col].character != 0 {
            should_break = true
        }

        self.term_buffer[buff_len - 1][col] = ScreenChar::null();
        Ok(should_break)
    }

    fn internal_newline(&mut self) { // moves all content one line up the screen and creates new line
        if self.application_mode { return; }; // only in terminal mode
        self.term_buffer.push([ScreenChar::null(); BUFFER_WIDTH]);
        self.col_pos = 0;
        if self.term_buffer.len() > 100 {
            self.term_buffer.remove(0);
        }
    }

    fn internal_lastline(&mut self) { // goes back to previous line and shifts all lines down
        if self.application_mode { return; };
        if self.term_buffer.len() <= 25 {
            self.term_buffer.insert(0, [ScreenChar::null(); BUFFER_WIDTH]);
        }
        self.term_buffer.pop();
        self.col_pos = BUFFER_WIDTH;
    }

    fn write_screen_char(&mut self, ch: ScreenChar) { // TODO: optimise so that screen is not fully re-rendered for every string written.
        match ch.character as u8 {
            b'\n' => self.internal_newline(),
            _ => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.internal_newline();
                }
                let _row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                let buff_len = self.term_buffer.len();
                self.term_buffer[buff_len - 1][col] = ch;
                self.col_pos += 1;
            }
        }
    }

    fn write_byte(&mut self, byte: u8, col: Option<ColorCode>) { // default colour if no colour is selected for character
        self.write_screen_char(ScreenChar {
            character: byte,
            colour: match col {
                Some(c) => c,
                None => match self.temp_colour {
                    Some(c) => c,
                    None => ColorCode::new(Color::White, Color::Black),
                }
            },
        });
    }

    fn internal_render(&mut self) { // private function that can only be used from within this struct.
        if self.application_mode {
            for (i, row) in self.app_buffer.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    self.screen_ref.chars[i][j].write(*col);
                }
            }
        } else {
            let buff_len = self.term_buffer.len();
            for (i, row) in self.term_buffer[buff_len - BUFFER_HEIGHT..buff_len].iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    self.screen_ref.chars[i][j].write(*col);
                }
            }
            self.internal_set_cursor_position(self.col_pos as u8, BUFFER_HEIGHT as u8 - 1);
        }
    }
}

pub fn special_char(ch: char) -> Option<u8> {
    let res: u8 = match ch {
        '│' => 179,
        '─' => 196,
        '┴' => 193,
        '┤' => 180,
        '═' => 205,
        '║' => 186,
        '╗' => 187,
        '╝' => 188,
        '╚' => 200,
        '╔' => 201,
        '»' => 175,
        '┐' => 191,
        '└' => 192,
        '┘' => 217,
        '┌' => 218,
        '┼' => 197,
        '░' => 176,
        '▒' => 177,
        '▓' => 178,
        '«' => 174,
        _ => {
            return None;
        }
    };
    Some(res)
}


impl fmt::Write for Renderer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string, None);
        Ok(())
    }
}

pub fn write(args: fmt::Arguments, cols: (Color, Color)) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let colour_code = ColorCode::new(cols.0, cols.1);

    interrupts::without_interrupts(|| {
        let mut writer = RENDERER.lock();

        writer.set_colour(colour_code);
        writer.write_fmt(args).unwrap();
        writer.reset_colour();
    })
}