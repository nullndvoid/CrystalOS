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
use crate::user::lib::libgui::cg_widgets::{CgContainer, CgIndicatorWidget, CgLabel};

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
	pub hit: bool,
	pub difficulty_idx: u8,
	pub gamespeed: f64,
}

#[async_trait]
impl Application for Game {
	fn new() -> Self {
		Self {
			player: Player::new(),
			enemies: Vec::new(),
			score: 0,
			hit: false,
			difficulty_idx: 1,
			gamespeed: 1.0,
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

		let score_label = Widget::insert(CgLabel::new(
			String::new(),
			Position::new(1, 1),
			78,
			true,
		));

		let self_ref = Widget::insert(self.clone());
		container_data.insert("app", self_ref);
		container_data.insert("score_label", score_label);
		let self_ref = container_data.fetch("app").unwrap();
		let score_ref = container_data.fetch("score_label").unwrap();

		loop {
			match self.score {
				0..=9 => { self.gamespeed = 1.0; self.difficulty_idx = 1 },
				10..=24 => { self.gamespeed = 2.0; self.difficulty_idx = 2 },
				25..=49 => { self.gamespeed = 3.0; self.difficulty_idx = 3 },
				50..=99 => { self.gamespeed = 4.0; self.difficulty_idx = 4 },
				100..=199 => { self.gamespeed = 5.0; self.difficulty_idx = 5 },
				_ => self.gamespeed = 10.0,
			};

			std::time::wait(0.2 / self.gamespeed);

			spawn_timer += 1;

			if spawn_timer >= 8 {
				spawn_timer = 0;
				self.new_enemy();
			}

			self.enemies.iter().for_each(|e| {

			});

			self.enemies.retain(|e| {
				if e.position.y == self.player.position.y && (0..5).map(|i| e.position.x + i).collect::<Vec<_>>().contains(&self.player.position.x) {
					self.player.health -= 1;
					self.hit = true;
					false
				} else if e.position.x <= 0 {
					self.score += 1;
					false
				} else {
					true
				}
			});
			self.enemies.iter_mut().for_each(|e| e.position.x -= 1);

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
			score_ref.update(CgLabel::new(format!("< Score: {} >", self.score), Position::new(1, 1), 78, true, ));
			if let Ok(frame) = container_data.render() {
				frame.write_to_screen().unwrap();
			}
			self.hit = false;
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

impl Game {
	fn new_enemy(&mut self) {

		let enemy_num = match self.difficulty_idx {
			1 => 1,
			2 => 2,
			3 => 3,
			4 => 5,
			_ => 7,
		};

		for _ in 0..enemy_num {
			self.enemies.push(Enemy::new(Random::int(1, 21)));
		}
	}
}

impl CgComponent for Game {
	fn render(&self) -> Result<Frame, RenderError> {
		let mut frame = Frame::new(Dimensions::new(1, 2), Dimensions::new(78, 22))?;

		let pos = self.player.position;

		let player_colour = match self.hit {
			true => Color::Red,
			false => Color::Cyan
		};

		frame[pos.y][pos.x] = ColouredChar {
			character: '@',
			colour: ColorCode::new(player_colour, Color::Black),
		};

		for i in self.enemies.iter().map(|enemy| enemy.position).collect::<Vec<Position>>() {
			frame[i.y][i.x] = ColouredChar {
				character: '<',
				colour: ColorCode::new(Color::LightGray, Color::Black),
			};
			(1..5).for_each(|offset| if i.x + offset < frame.dimensions.x { frame[i.y][i.x + offset] = ColouredChar {
				character: '=',
				colour: ColorCode::new(Color::LightGray, Color::Black),
			}});
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

