use crate::{
    kernel::render::{RENDERER, self},
    kernel::tasks::keyboard::KEYBOARD,
};

use alloc::string::String;
use alloc::vec::Vec;

pub use crate::{print, println, serial_print, serial_println};
pub use crate::kernel::render::Color;

use lazy_static::lazy_static;
use spin::Mutex;

pub struct Stdin {}
impl Stdin {
    pub async fn readline() -> String {
        let string = KEYBOARD.lock().get_string().await;
        string
    }

    pub async fn keystroke() -> char {
        let chr = KEYBOARD.lock().get_keystroke().await;
        chr
    }

    pub fn try_keystroke() -> Option<char> {
        let chr = KEYBOARD.lock().try_keystroke();
        chr
    }
}

pub struct Screen {}
impl Screen {
    pub fn terminal_mode() {
        RENDERER.lock().terminal_mode().unwrap();
    }
    pub fn application_mode() {
        RENDERER.lock().application_mode().unwrap();
    }
    pub fn switch() {
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