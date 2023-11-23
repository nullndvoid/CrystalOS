use crate::{
    kernel::render::{RENDERER, self},
    kernel::tasks::keyboard::{KEYBOARD, KeyStroke},
};

use alloc::string::String;
use alloc::vec::Vec;

pub use crate::{print, println, serial_print, serial_println};
pub use crate::kernel::render::Color;
use crate::kernel::serial::serial_reply;

use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::render::Renderer;
use crate::std::frame::RenderError;

pub struct Stdin {}
impl Stdin {
    /// waits for the user to type in a string and press enter | blocking
    pub async fn readline() -> String {
        let string = KEYBOARD.lock().get_string().await;
        string
    }

    /// waits for a keystroke | blocking
    pub async fn keystroke() -> char {
        let chr = KEYBOARD.lock().get_keystroke().await;
        chr
    }

    /// gets the next keystroke if any is present | non blocking
    pub fn try_keystroke() -> Option<char> {
        let chr = KEYBOARD.lock().try_keystroke();
        chr
    }
}

pub struct Serial {}

impl Serial {
    pub fn reply_char(c: char) -> char {
        serial_reply(c)
    }
}

/// enum with a terminal and application mode
pub enum Screen {
    Terminal,
    Application,
}
impl Screen {
    /// mode can be set for the kernel using this method
    pub fn set_mode(&self) -> Result<(), RenderError> {
        match self {
            Screen::Terminal => RENDERER.lock().terminal_mode(),
            Screen::Application => RENDERER.lock().application_mode(),
        }
    }

    /// returns the current display mode
    pub fn get_mode() -> Screen {
        match RENDERER.lock().mode_is_app() {
            true => Screen::Application,
            false => Screen::Terminal,
        }
    }

    /// switches between modes
    pub fn switch(&self) {
        if RENDERER.lock().mode_is_app() == true {
            RENDERER.lock().terminal_mode().unwrap();
        } else {
            RENDERER.lock().application_mode().unwrap();
        }
    }
    pub fn clear() {
        RENDERER.lock().clear();
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

#[macro_export]
macro_rules! printerr {
    ($($arg:tt)*) => ($crate::std::io::_printerr(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	render::write(args, (Color::White, Color::Black));
}

#[doc(hidden)]
pub fn _printerr(args: core::fmt::Arguments) {
    render::write(args, (Color::Yellow, Color::Black));
}

#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
    render::write(args, (Color::White, Color::Black));
}

pub fn write(args: core::fmt::Arguments, color: (Color, Color)) {
    render::write(args, color);
}