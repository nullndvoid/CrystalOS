use crate::system::std::application::Application;
use async_trait::async_trait;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use crate::std::application::Error;

pub struct Player {
	pub health: u32,
	pub score: u32
}
impl Player {
	pub fn new() -> Player {
		Player {
			health: 5,
			score: 0
		}
	}
}

pub struct Game {
	pub player: Player
}
