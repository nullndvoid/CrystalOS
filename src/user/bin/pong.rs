use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use async_trait::async_trait;
use crate::std::application::{Application, Error};
use crate::std;
use crate::std::frame::{ColouredChar, Dimensions, Frame, Position, RenderError};
use crate::std::io::{Display, KeyStroke, Stdin};
use crate::user::lib::libgui::cg_core::CgComponent;

pub(crate) struct Game {
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
            player2: Player::new(78),
        }
    }

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        let d = Display::borrow();

        loop {
            std::time::wait(0.01);

            if let Some(key) = Stdin::try_keystroke() {
                match key {
                    KeyStroke::Char('w') => {
                        self.player1.pos.y -= 1;
                    },
                    KeyStroke::Char('s') => {
                        self.player1.pos.y += 1;
                    },
                    KeyStroke::Up => {
                        self.player2.pos.y -= 1;
                    },
                    KeyStroke::Down => {
                        self.player2.pos.y += 1;
                    }
                    _ => {}
                }
            }
            if let Ok(frame) = self.render() {
                frame.write_to_screen().unwrap();
            }
            // self.ball.update(&self.player1, &self.player2);
        }
        Ok(())
    }
}

impl CgComponent for Game {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Dimensions::new(0, 0), Dimensions::new(80, 25))?;
        
        frame.write(self.player1.pos, ColouredChar::new('@')).unwrap();
        frame.write(self.player2.pos, ColouredChar::new('@')).unwrap();
        frame.write(self.ball.pos, ColouredChar::new('@')).unwrap();

        Ok(frame)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Player {
    pos: Position,
    score: i32,
}

impl Player {
    fn new(x: usize) -> Self {
        Player { pos: Position::new(x, 12), score: 0 }
    }
}

struct Ball {
    pos: Position,
    vx: i32,
    vy: i32,
}

impl Ball {
    fn new() -> Self {
        Ball {  pos: Position::new(40, 12), vx: 1, vy: 0 }
    }
    
    fn update(&mut self, player1: &Player, player2: &Player) {
        self.pos.x = (self.pos.x as i32 + self.vx) as usize;
    }
}
