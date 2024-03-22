use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use async_trait::async_trait;
use crate::std::application::{Application, Error};
use crate::std;
use crate::std::frame::{BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, ColouredChar, Dimensions, Frame, Position, RenderError};
use crate::std::io::{Color, Display, KeyStroke, Stdin};
use crate::std::time::Timer;
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

        let mut update_time = Timer::new(0.1);

        let mut updated;
        loop {
            updated = false;

            if let Some(key) = Stdin::try_keystroke() {
                match key {
                    KeyStroke::Char('w') => {
                        self.player1.move_player(-1);
                        updated = true;
                    },
                    KeyStroke::Char('s') => {
                        self.player1.move_player(1);
                        updated = true;
                    },
                    KeyStroke::Up => {
                        self.player2.move_player(-1);
                        updated = true;
                    },
                    KeyStroke::Down => {
                        self.player2.move_player(1);
                        updated = true;
                    },
                    KeyStroke::Char('`') => break,
                    _ => {}
                }
            }
            if update_time.is_done() {
                updated = true;
                self.ball.update(&mut self.player1, &mut self.player2);
                update_time.reset()
            }

            if updated {
                if let Ok(frame) = self.render() {
                    frame.write_to_screen().unwrap();
                }
            }
            // self.ball.update(&self.player1, &self.player2);
        }
        Ok(())
    }
}

impl CgComponent for Game {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Dimensions::new(0, 0), Dimensions::new(80, 25))?;

        for y in (0..5) {
            frame.write(Position::new(self.player1.pos.x, self.player1.pos.y + y -2), ColouredChar::coloured('▓', ColorCode::new(Color::Cyan, Color::Black))).unwrap();
            frame.write(Position::new(self.player2.pos.x, self.player2.pos.y + y -2), ColouredChar::coloured('▓', ColorCode::new(Color::Cyan, Color::Black))).unwrap();
        }
        frame.write(self.ball.pos, ColouredChar::coloured('O', ColorCode::new(Color::Green, Color::Black))).unwrap();

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

    // valid for |y| = 1
    fn move_player(&mut self, y: i32) {
        if self.pos.y < 3 && y < 0 {
            return;
        } else if self.pos.y >= BUFFER_HEIGHT - 3 && y > 0 {
            return;
        }
        self.pos.y = (self.pos.y as i32 + y) as usize;
    }
}

struct Ball {
    pos: Position,
    vx: i32,
    vy: i32,
}

impl Ball {
    fn new() -> Self {
        Ball {  pos: Position::new(40, 12), vx: 1, vy: 1 }
    }
    
    fn update(&mut self, player1: &mut Player, player2: &mut Player) {
        let pos_next = Position::new( // invert x direction on collision with player
            (self.pos.x as i32 + self.vx) as usize,
            (self.pos.y as i32 + self.vy) as usize
        );
        for i in 0..5 {
            if player1.pos.y + i == pos_next.y && player1.pos.x == pos_next.x {
                self.vx = -self.vx;
                break;
            } else if player2.pos.y + i == pos_next.y && player2.pos.x == pos_next.x {
                self.vx = -self.vx;
                break;
            }
        };

        if pos_next.y < 0 || pos_next.y >= BUFFER_HEIGHT { // if the move is outside the screen, then invert the direction
            self.vy = -self.vy;
        }

        if pos_next.x < 0 {
            player2.score += 1;
            self.pos = Position::new(40, 12);
            self.vx = 1;
            self.vy = 1;
        }

        if pos_next.x >= BUFFER_WIDTH {
            player1.score += 1;
            self.pos = Position::new(40, 12);
            self.vx = -1;
            self.vy = 1;
        }

        self.pos = Position::new(
            (self.pos.x as i32 + self.vx) as usize,
            (self.pos.y as i32 + self.vy) as usize
        )
    }
}
