use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;


use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{println, serial_println};

use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::Stream;
use futures_util::task::AtomicWaker;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use crate::print;
use crate::kernel::render::RENDERER;
use alloc::{string::String};

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
	None,
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
			_ => KeyStroke::None,
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
				if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
					if let Some(key) = self.keyboard.process_keyevent(key_event) {
						match key {
							DecodedKey::Unicode(character) => return Some(KeyStroke::Char(character)),
							DecodedKey::RawKey(key) => {
								print!("{:?}", key);
								match KeyStroke::from_keycode(key) {
									KeyStroke::None => (),
									k => return Some(k)
								}
							},
						}
					}
				}
			}
		}
	}

	pub async fn get_keystroke(&mut self) -> KeyStroke {
		loop {
			match self.get_keystroke_inner().await {
				Some(c) => match c {
					KeyStroke::None => (),
					c => return c
				},
				None => ()
			}
		}
	}

	pub fn try_keystroke(&mut self) -> Option<KeyStroke> {
		if let Some(scancode) = self.scancodes.try_next() {
			if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
				if let Some(key) = self.keyboard.process_keyevent(key_event) {
					match key {
						DecodedKey::Unicode(character) => {
							if character == b'\x08' as char { // checks if the character is a backspace
								interrupts::without_interrupts(|| {
									RENDERER.lock().backspace(); // runs the backspace function of the vga buffer to remove the last character
								});
								return None;
							} else {
								return Some(KeyStroke::Char(character));
							}
						},
						DecodedKey::RawKey(key) => {
							print!("{:?}", key);
							match KeyStroke::from_keycode(key) {
								KeyStroke::None => (),
								key => return Some(key)
							}
						},
					}
				}
			}
		};
		None
	}


	pub async fn get_string(&mut self) -> String {
		let mut val = String::new();
		loop {
			let character = match self.get_keystroke_inner().await {
				Some(c) => { c },
				None => { val.pop(); continue; },
			};

			if let KeyStroke::Char(c) = character {
				if c == '\x08' {
					val.pop();
					interrupts::without_interrupts(|| {
						RENDERER.lock().backspace(); // runs the backspace function of the vga buffer to remove the last character
					});
					continue;
				}

				print!("{}", c);
				let (c, execute): (char, bool) = match c {
					'\n' => (c, true),
					_ => (c, false),
				};
				val.push(c);
				if execute {
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
		SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
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
			},
			Err(crossbeam_queue::PopError) => Poll::Pending,
		}
	}
}
