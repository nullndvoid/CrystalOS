use alloc::string::String;
use alloc::{format, vec, vec::Vec, boxed::Box};
use alloc::borrow::ToOwned;
use core::arch::x86_64::_mm_test_all_ones;
use core::cell::RefCell;
use async_trait::async_trait;
use crate::std::io::{Color, Screen, Stdin};
use crate::std::time;
use crate::kernel::tasks::keyboard::KEYBOARD;
use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;
use crate::kernel::render::{ColorCode, ScreenChar};
use crate::{println};
use crate::std::application::{Application, Error};
use crate::std::random::Random;
use crate::system::std::frame::ColouredElement;
use super::super::lib::coords::{Line, Position, Direction};


#[derive(Clone, Debug, PartialEq)]
enum Status {
    Scored,
    Lost,
    Exited,
    None,
}

pub struct Game {
    snakes: Vec<Snake>,
    pois: Vec<Position>,
    score: u8,
    hardmode: bool,
}



#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Self {
            snakes: Vec::new(),
            pois: Vec::new(),
            score: 0,
            hardmode: false,
        }
    }

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        //Screen::application_mode();


        // make the first poi

        self.snakes.push(Snake::player(0));

        for i in 0..5 {
            self.new_poi();
            self.snakes.push(Snake::ai(i + 1));
        }

        // render the initial state of the screen.
        self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;

        // run the game
        self.gameloop().await?;

        // return to the terminal
        //Screen::terminal_mode();
        Ok(())
    }
}

impl Game {

    async fn gameloop(&mut self) -> Result<(), Error> { // main gameloop
        let mut all_points: Vec<Position>;

        'gameloop: loop {

            time::wait(0.1);

            let mut points: Vec<Position>;
            let length = self.snakes.len();

            for i in 0..length {
                let points: Vec<Position> = self.snakes.clone().into_iter().map(|s| s.tail).flatten().collect();
                let res = self.snakes[i].next(&self.pois, &points);

                match res {
                    Status::Lost => {
                        if !self.snakes[i].ai_controlled {
                            self.render_end_screen().map_err(|_| Error::ApplicationError(String::from("failed to render end screen")))?;
                            // loop triggers when game is lost
                            loop {
                                match Stdin::keystroke().await {
                                    'x' => break 'gameloop,
                                    _ => continue,
                                }
                            }
                        }
                    },
                    Status::Exited => {
                        break 'gameloop
                    },
                    Status::Scored => {
                        if !self.snakes[i].ai_controlled {
                            self.score += 1;
                        }
                        self.replace_poi(&self.snakes[i].clone().head); // passes a reference to the location of the current snake's head
                    },
                    Status::None => {},
                }
            }

            self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
        };
        Ok(())
    }

    fn new_poi(&mut self) {
        self.pois.push(Position { x: Random::int(3, 76) as i64, y: Random::int(3, 21) as i64 });
    }

    fn replace_poi(&mut self, poi: &Position) {
        self.pois.remove(self.pois.iter().position(|p| p == poi).unwrap());
        self.new_poi();
    }

    fn render(&mut self) -> Result<(), ()> {
        let mut frame = vec![vec![ScreenChar::null(); 80]; 25];
        self.snakes.clone().into_iter().map(|s| s.tail).flatten().for_each(|p| {
            frame[24 - p.y as usize][p.x as usize] = ScreenChar::new('@' as u8, ColorCode::new(Color::Cyan, Color::Black));
        });

        self.pois.iter().for_each(|poi| {
            frame[24 - poi.y as usize][poi.x as usize] = ScreenChar::new('o' as u8, ColorCode::new(Color::Red, Color::Black));
        });

        let literal = format!("snake go brr score:  {}", self.score);
        let msg = Game::centre_text(80, literal);
        frame[1] = msg.chars().map(|c| ScreenChar::new(c as u8, ColorCode::new(Color::LightGreen, Color::Black))).collect();

        let mut elem = ColouredElement::generate(frame, (80, 25));
        elem.render((0,0));

        Ok(())
    }


    fn centre_text(dims: usize, text: String) -> String { // centres text in a string of whitespace of a given length
        let max_pad = dims / 2;
        let mut msg = String::new();
        msg.push_str(" ".repeat(max_pad - round_up(text.len() as f64 / 2.0)).as_str());
        msg.push_str(text.as_str());
        msg.push_str(" ".repeat(max_pad - round_down(text.len() as f64 / 2.0 + 0.51)).as_str());
        msg
    }

    fn render_end_screen(&mut self) -> Result<(), ()> {
        let mut frame = vec![vec![ScreenChar::null(); 80]; 25];

        frame[10] = Game::centre_text(80, String::from("u lost")).chars().map(|c| ScreenChar::new(c as u8, ColorCode::new(Color::Red, Color::Black))).collect();
        frame[12] = Game::centre_text(80, String::from(format!("ur score was {}", self.score))).chars().map(|c| ScreenChar::new(c as u8, ColorCode::new(Color::LightGreen, Color::Black))).collect();
        frame[14] = Game::centre_text(80, String::from("L bozo")).chars().map(|c| ScreenChar::new(c as u8, ColorCode::new(Color::Red, Color::Black))).collect();

        let mut elem = ColouredElement::generate(frame, (80, 25));
        elem.render((0,0))
    }
}












#[derive(Debug, Clone)]
struct Snake {
    ai_controlled: bool,
    head: Position,
    tail: Vec<Position>,
    dir: Direction,
}

impl Snake {
    fn ai(id: usize) -> Self {
        Self {
            ai_controlled: true,
            head: Position { x: 4 + 4*id as i64 * 2, y: 9 },
            tail: (1..4).map(|p| Position { x: 4 + 4*id as i64, y: 5 + p}).collect(),
            dir: Direction::PosY,
        }
    }
    fn player(id: usize) -> Self {
        Self {
            ai_controlled: false,
            head: Position { x: 4 + 4*id as i64, y: 9 },
            tail: (1..4).map(|p| Position { x: 4 + 4*id as i64, y: 5 + p}).collect(),
            dir: Direction::PosY,
        }
    }

    fn next(&mut self, points_of_interest: &Vec<Position>, tails: &Vec<Position>) -> Status {  // returns (lose_condition, scored)

        // uses pathing algorithm if ai else keyboard input if human
        if self.ai_controlled {
            self.dir = PathFinder::decide(&self.head, points_of_interest, tails);
        } else {
            // if let Some(c) = Stdin::try_keystroke() {
            //     self.dir = match c {
            //         'w' => Direction::PosY,
            //         'a' => Direction::NegX,
            //         's' => Direction::NegY,
            //         'd' => Direction::PosX,
            //         'x' => return Status::Exited,
            //         _ => self.dir.clone(),
            //     }
            // }
            self.dir = Direction::None;
        }

        if self.dir != Direction::None {
            self.tail.push(self.head.clone());
        }

        match self.dir {
            Direction::PosY => self.head.y += 1,
            Direction::NegY => self.head.y -= 1,
            Direction::NegX => self.head.x -= 1,
            Direction::PosX => self.head.x += 1,
            Direction::None => {},
        }

        if self.lose_condition(tails) {
            self.tail.remove(0);
            return Status::Lost;
        }

        if points_of_interest.contains(&self.head) {
            return Status::Scored;
        } else {
            if self.dir != Direction::None {
                self.tail.remove(0);
            }
        }

        Status::None
    }

    fn lose_condition(&mut self, tails: &Vec<Position>) -> bool { // where tails includes the tail of every other snake
        let p = self.head.clone();
        let snake_overlaps = tails.contains(&self.head); // checks if any part of the snake overlaps itself
        let out_of_bounds = p.x < 0 || p.y < 0 || p.x > 79 || p.y > 24; // checks if the snake goes out of bounds

        snake_overlaps || out_of_bounds
    }
}

struct PathFinder {}

impl PathFinder {
    fn decide(head: &Position, tails: &Vec<Position>, pois: &Vec<Position>) -> Direction {
        let nearest_poi = head.nearest(pois);
        let rel_pos = head.distance(&nearest_poi);

        // check actions don't lose them the game
        let mut possible_moves = Vec::new();
        let mut h: Position;

        h = Position { x: head.x + 1, y: head.y };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::PosX);
        }
        h = Position { x: head.x - 1, y: head.y };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::NegX);
        }
        h = Position { x: head.x, y: head.y + 1 };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::PosY);
        }
        h = Position { x: head.x, y: head.y - 1 };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::NegY);
        }

        if possible_moves.is_empty() {
            panic!("no possible moves");
            return Direction::None;
        } else {
            let optimal = PathFinder::optimal_move(head, &rel_pos, &possible_moves);
            println!("{:?} {:?} {:?} {:?}", nearest_poi, rel_pos, head, optimal);
        }

        Direction::None
    }

    fn optimal_move(head: &Position, rel_pos: &Position, moves: &Vec<Direction>) -> Direction {
        let mut optimal_moves = vec![Direction::None; 4];

        let x_offset: usize;
        let y_offset: usize;

        if rel_pos.x.abs() > rel_pos.y.abs() {
            y_offset = 1;
            x_offset = 0;
        } else {
            x_offset = 1;
            y_offset = 0;
        }

        if rel_pos.x < 0 {
            optimal_moves[x_offset] = Direction::NegX;
            optimal_moves[x_offset + 2] = Direction::PosX;
        } else {
            optimal_moves[x_offset] = Direction::PosX;
            optimal_moves[x_offset + 2] = Direction::NegX;
        }

        if rel_pos.y < 0 {
            optimal_moves[y_offset] = Direction::NegY;
            optimal_moves[y_offset + 2] = Direction::PosY;
        } else {
            optimal_moves[y_offset] = Direction::PosY;
            optimal_moves[y_offset + 2] = Direction::NegY;
        }
        //println!("moves: {:?}, optimal_moves: {:?}, rel_pos: {:?}", moves, optimal_moves, rel_pos);
        for m in optimal_moves {
            if moves.contains(&m) {
                return m;
            }
        };

        // this should never be used, the above statement should always return a value.
        panic!("No optimal move found (this should not happen)");
    }


    fn check_bounds(head: &Position) -> bool {
        head.x < 0 || head.y < 0 || head.x > 79 || head.y > 24
    }

    fn check_collision(head: &Position, tails: &Vec<Position>) -> bool {
        tails.contains(&head)
    }
}





fn round_up(n: f64) -> usize {
    (n + 0.99) as usize
}
fn round_down(n: f64) -> usize {
    n as usize
}

