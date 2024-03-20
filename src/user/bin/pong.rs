use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use async_trait::async_trait;
use core::fmt::Write;
use crate::std::application::{Application, Error};
use crate::std;

struct Game {
    ball: Ball,
    player1: Player,
    player2: Player,
}

#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Game {
            ball: Ball::new(),
            player1: Player::new(1),
            player2: Player::new(2),
        }
    }

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        loop {
            self.ball.update(&self.player1, &self.player2);
        }
        Ok(())
    }
}

struct Player {
    x: i32,
    y: i32,
    score: i32,
}

impl Player {
    fn new(y: i32) -> Self {
        Player { x: 0, y, score: 0 }
    }
}

struct Ball {
    x: i32,
    y: i32,
}

impl Ball {
    fn new() -> Self {
        Ball { x: 0, y: 0 }
    }
    fn update(&mut self, player1: &Player, player2: &Player) {
        self.x += 1;
    }
}
