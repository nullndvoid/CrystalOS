use crate::system::std::application::Application;
use async_trait::async_trait;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::any::Any;
use crate::{serial_println, std};
use crate::std::application::Error;
use crate::std::application::Error::ApplicationError;
use crate::std::frame::{Position, Dimensions, RenderError, Frame, ColouredChar};
use crate::std::io::{Color, ColorCode, KeyStroke, Screen, Stdin};
use crate::std::random::Random;
use crate::user::lib::libgui::cg_core::{CgComponent, Widget};
use crate::user::lib::libgui::cg_widgets::{CgContainer, CgIndicatorWidget};

#[derive(Clone)]
pub struct Player {
	pub health: u32,
	pub position: Position,
}
impl Player {
	pub fn new() -> Player {
		Player {
			health: 5,
			position: Position::new(10, 12)
		}
	}
}

#[derive(Clone)]
pub struct Game {
	pub player: Player,
	pub enemies: Vec<Enemy>,
	pub score: u32,
}

#[async_trait]
impl Application for Game {
	fn new() -> Self {
		Self {
			player: Player::new(),
			enemies: Vec::new(),
			score: 0
		}
	}
	async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
		let mut spawn_timer: i32 = 0;

		Screen::Application.set_mode().unwrap();

		let mut container_data = CgContainer::new(
			Position::new(0, 0),
			Dimensions::new(80, 25),
			true,
		);

		let self_ref = Widget::insert(self.clone());
		container_data.insert("app", self_ref);
		let self_ref = container_data.fetch("app").unwrap();

		loop {
			std::time::wait(0.1);

			spawn_timer += 1;

			if spawn_timer >= 8 {
				spawn_timer = 0;
				self.enemies.push(Enemy::new(Random::int(1, 21)));
			}

			self.enemies.retain(|e| {if e.position.x <= 0 {self.score += 1; false } else { true }});
			self.enemies.iter_mut().for_each(|e| e.position.x -= 1);
			self.enemies.iter().for_each(|e| { if e.position.x == self.player.position.x && e.position.y == self.player.position.y { self.player.health -= 1; } });

			if self.player.health == 0 {
				break;
			}

			if let Some(input_key) = Stdin::try_keystroke() {
				match input_key {
					KeyStroke::Char('q') => {
						break;
					}
					KeyStroke::Char('w') => self.player.position.y -= 1,
					KeyStroke::Char('s') => self.player.position.y += 1,
					KeyStroke::Char('a') => self.player.position.x -= 1,
					KeyStroke::Char('d') => self.player.position.x += 1,
					_ => (),
				}
			}
			self_ref.update(self.clone());
			if let Ok(frame) = container_data.render() {
				frame.write_to_screen().unwrap();
			}
		}

		let mut frame = Frame::new(Dimensions::new(0, 0), Dimensions::new(80, 25)).map_err(|_| ApplicationError("idk".to_string()))?;
		let msg = format!("your score was: {}", self.score);
		msg.chars().enumerate().for_each(|(i, c)| {
			serial_println!("{}", (80 - msg.len()) / 2 + i);
			frame[12][(80 - msg.len()) / 2 + i] = ColouredChar {
				character: c,
				colour: ColorCode::new(Color::Cyan, Color::Black),
			}
		});
		frame.write_to_screen().unwrap();

		while let KeyStroke::Char(c) = Stdin::keystroke().await {
			if c == 'q' {
				break;
			}
		}

		Screen::Terminal.set_mode().unwrap();
		Ok(())
	}
}

impl CgComponent for Game {
	fn render(&self) -> Result<Frame, RenderError> {
		let mut frame = Frame::new(Dimensions::new(1, 1), Dimensions::new(78, 23))?;

		let pos = self.player.position;

		frame[pos.y][pos.x] = ColouredChar {
			character: '@',
			colour: ColorCode::new(Color::Cyan, Color::Black),
		};

		for i in self.enemies.iter().map(|enemy| enemy.position).collect::<Vec<Position>>() {
			frame[i.y][i.x] = ColouredChar {
				character: '*',
				colour: ColorCode::new(Color::Red, Color::Black),
			}
		}

		Ok(frame)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}


#[derive(Clone)]
pub struct Enemy {
	pub position: Position
}

impl Enemy {
	pub fn new(y: usize) -> Enemy {
		Enemy {
			position: Position::new(75, y)
		}
	}
}

