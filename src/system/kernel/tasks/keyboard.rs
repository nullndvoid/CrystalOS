use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::{println, serial_print, serial_println, system::kernel::serial};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use crate::print;
use crate::system::kernel::render::RENDERER;
use alloc::string::String;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

static WAKER: AtomicWaker = AtomicWaker::new();
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

lazy_static! {
    pub static ref KEYBOARD: Mutex<KeyboardHandler> = Mutex::new(KeyboardHandler::new());
}

pub struct KeyboardHandler {
    scancodes: ScanCodeStream,
    keyboard: Keyboard<layouts::Uk105Key, ScancodeSet1>,
}

enum CharOrKeystroke {
    Char(char),
    Keystroke(KeyCode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyStroke {
    Char(char),
    Ctrl,
    RCtrl,
    Alt,
    RAlt,
    Shift,
    RShift,
    Meta,
    RMeta,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    None,
    Enter,
    Escape,
    Del,
}

impl KeyStroke {
    pub fn from_keycode(key: KeyCode) -> KeyStroke {
        match key {
            KeyCode::ControlLeft => KeyStroke::Ctrl,
            KeyCode::ControlRight => KeyStroke::RCtrl,
            KeyCode::AltLeft => KeyStroke::Alt,
            KeyCode::AltRight => KeyStroke::RAlt,
            KeyCode::ShiftLeft => KeyStroke::Shift,
            KeyCode::ShiftRight => KeyStroke::RShift,
            KeyCode::WindowsLeft => KeyStroke::Meta,
            KeyCode::WindowsRight => KeyStroke::RMeta,
            KeyCode::Backspace => KeyStroke::Backspace,
            KeyCode::ArrowLeft => KeyStroke::Left,
            KeyCode::ArrowRight => KeyStroke::Right,
            KeyCode::ArrowUp => KeyStroke::Up,
            KeyCode::ArrowDown => KeyStroke::Down,
            KeyCode::Enter => KeyStroke::Enter,
            KeyCode::Escape => KeyStroke::Escape,
            KeyCode::Delete => KeyStroke::Del,
            _ => KeyStroke::None,
        }
    }
}

impl core::fmt::Display for KeyStroke {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KeyStroke::Char(c) => write!(f, "{}", c),
            KeyStroke::Ctrl => write!(f, "CTRL"),
            KeyStroke::RCtrl => write!(f, "RCtrl"),
            KeyStroke::Alt => write!(f, "ALT"),
            KeyStroke::RAlt => write!(f, "RAlt"),
            KeyStroke::Shift => write!(f, "SHIFT"),
            KeyStroke::RShift => write!(f, "RShift"),
            KeyStroke::Meta => write!(f, "META"),
            KeyStroke::RMeta => write!(f, "RMeta"),
            KeyStroke::Backspace => write!(f, "BACKSPACE"),
            KeyStroke::Left => write!(f, "LEFT"),
            KeyStroke::Right => write!(f, "RIGHT"),
            KeyStroke::Up => write!(f, "UP"),
            KeyStroke::Down => write!(f, "DOWN"),
            KeyStroke::Enter => write!(f, "ENTER"),
            KeyStroke::Escape => write!(f, "ESCAPE"),
            KeyStroke::None => write!(f, "NONE"),
            KeyStroke::Del => write!(f, "DEL"),
        }
    }
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        KeyboardHandler {
            scancodes: ScanCodeStream::new(),
            keyboard: Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore),
        }
    }

    pub async fn get_keystroke_inner(&mut self) -> Option<KeyStroke> {
        loop {
            if let Some(scancode) = self.scancodes.next().await {
                return self.process_keystroke(scancode);
            }
        }
    }

    pub fn process_keystroke(&mut self, scancode: u8) -> Option<KeyStroke> {
        if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
            if let Some(key) = self.keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == b'\x08' as char {
                            // checks if the character is a backspace
                            interrupts::without_interrupts(|| {
                                RENDERER.lock().backspace(); // runs the backspace function of the vga buffer to remove the last character
                            });
                            return Some(KeyStroke::Char(character));
                        } else {
                            return Some(KeyStroke::Char(character));
                        }
                    }
                    DecodedKey::RawKey(key) => match KeyStroke::from_keycode(key) {
                        KeyStroke::None => (),
                        key => return Some(key),
                    },
                }
            }
        }
        None
    }

    pub async fn get_keystroke(&mut self) -> KeyStroke {
        loop {
            if let Some(c) = self.scancodes.next().await {
                if let Some(key) = self.process_keystroke(c) {
                    return key;
                }
            }
        }
    }

    pub fn try_keystroke(&mut self) -> Option<KeyStroke> {
        if let Some(scancode) = self.scancodes.try_next() {
            self.process_keystroke(scancode)
        } else {
            None
        }
    }

    pub fn last_keystroke(&mut self) -> Option<KeyStroke> {
        let mut last_scancode = None;

        // Keep getting scancodes until the queue is empty
        while let Some(scancode) = self.scancodes.try_next() {
            last_scancode = Some(scancode);
        }

        // Process the last scancode
        if let Some(scancode) = last_scancode {
            self.process_keystroke(scancode)
        } else {
            None
        }
    }

    pub async fn get_string(&mut self) -> String {
        let mut val = String::new();
        loop {
            let character = match self.get_keystroke_inner().await {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            if let KeyStroke::Char(c) = character {
                if c == '\x08' {
                    val.pop();
                    continue;
                }

                print!("{}", c);
                val.push(c);

                if c == '\n' {
                    return val;
                }
            }
        }
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: queue is full - ignoring input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue has not been initialised");
    }
}

pub struct ScanCodeStream {
    _private: (),
}

impl ScanCodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScanCodeStream::new has already been called once");
        ScanCodeStream { _private: () }
    }

    pub fn try_next(&mut self) -> Option<u8> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialised");
        if let Ok(c) = queue.pop() {
            Some(c)
        } else {
            None
        }
    }
}

impl Stream for ScanCodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialised");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&ctx.waker());

        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}
