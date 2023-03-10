use crate::{
    kernel::render::{RENDERER, BUFFER_WIDTH, BUFFER_HEIGHT, ColorCode},
    kernel::tasks::keyboard::KEYBOARD,
};


use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};

pub use crate::{print, println, serial_print, serial_println};

use lazy_static::lazy_static;
use spin::Mutex;

pub async fn stdin() -> String {
    let string = KEYBOARD.lock().get_string().await;
    string
}

pub async fn stdchar() -> char {
    let chr = KEYBOARD.lock().get_keystroke().await;
    chr
}

pub fn text_mode() {
    RENDERER.lock().text_mode().unwrap();
}

pub fn sandbox_mode() {
    RENDERER.lock().sandbox_mode().unwrap();
}

pub fn switch_mode() {
    if RENDERER.lock().sandbox == true {
        RENDERER.lock().text_mode().unwrap();
    } else {
        RENDERER.lock().sandbox_mode().unwrap();
    }
}

pub fn clear() {
    RENDERER.lock().clear();
}





pub type Frame = [ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT];

#[derive(Clone)]
pub struct Element {
    frame: Vec<Vec<char>>,
    dimensions: (u8, u8)
}
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
/// 
/// nothing will appear on the screen until the frame is actually
impl Element {
    pub fn from_str(elemstr: String) -> Self {
        let mut element = Element { frame: Vec::<Vec<char>>::new(), dimensions: (0, 0) }; 

        for line in elemstr.split("\n") {
            let mut ln = Vec::<char>::new();
            for col in line.chars() {
                ln.push(col)
            };
            element.frame.push(ln);
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.0 as usize {
                element.dimensions.0 = n as u8;
            }
        }
        element
    }

    pub fn render(&mut self,  pos: (u8, u8)) { // x,y
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                println!("{} {} {}", i, j, col);
                FRAMEGEN.lock().frame[i + pos.1 as usize][j + pos.0 as usize] = *col;
            };
        }
    }
}


lazy_static! {
    pub static ref FRAMEGEN: Mutex<FrameGen> = Mutex::new(FrameGen::new() );
}


#[derive(Clone, Copy)]
pub struct FrameGen {
    frame: Frame,
}


impl FrameGen {
    pub fn render_frame(&self) {
        RENDERER.lock().render_frame(self.frame)
    }

    fn new() -> Self {
        let mut frame: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[' '; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for i in 0..BUFFER_WIDTH {
            frame[0][i] = "┌──────────────────────────────────────────────────────────────────────────────┐".chars().collect::<Vec<char>>()[i];
            frame[BUFFER_HEIGHT -1][i] = "└──────────────────────────────────────────────────────────────────────────────┘".chars().collect::<Vec<char>>()[i];
        }
        
        for j in 1..BUFFER_HEIGHT -1 {
            for i in 0..BUFFER_WIDTH {
                frame[j][i] = "│                                                                              │".chars().collect::<Vec<char>>()[i];               
            }
        }

        Self { frame: Frame::from(frame) }
    }

    pub fn get_frame(&self) -> &[ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT] {
        &self.frame
    }

}


impl core::fmt::Display for FrameGen {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        println!(" ");
        for row in &self.frame {
            println!("{}", row.iter().collect::<String>());
        };
        Ok(())
    }
}





#[macro_export]
macro_rules! println_log {
	() => ($crate::print_log!("/n"));
	($($arg:tt)*) => ($crate::print_log!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_log {
	($($arg:tt)*) => ($crate::std::io::_log(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
	() => ($crate::print!("/n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::std::io::_print(format_args!($($arg)*)));
}

pub use crate::kernel::render::Color;


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		let mut writer = RENDERER.lock();
		writer.col_code = ColorCode::new(Color::White, Color::Black);
		writer.write_fmt(args).unwrap();
		
		//WRITER.lock().write_fmt(args).unwrap();
	});
}

#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		let mut writer = RENDERER.lock();
		writer.col_code = ColorCode::new(Color::Yellow, Color::Black);
		writer.write_fmt(args).unwrap();
		
		//WRITER.lock().write_fmt(args).unwrap();
	});
}

pub fn write(args: core::fmt::Arguments, cols: (Color, Color)) {
    crate::kernel::render::write(args, cols);
}




pub fn mkfs() {
    use crate::kernel::fs;
    fs::mkfs();
    println!("{:?}", *(fs::FILESYSTEM.lock()));
}