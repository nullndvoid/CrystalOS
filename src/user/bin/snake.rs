use alloc::string::String;
use alloc::{format, vec, vec::Vec, boxed::Box};
use core::arch::x86_64::_mm_test_all_ones;
use core::cell::RefCell;
use async_trait::async_trait;
use crate::std::io::{Color, Screen, Stdin};
use crate::std::time;
use crate::kernel::tasks::keyboard::KEYBOARD;
use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;
use crate::kernel::render::{ColorCode, ScreenChar};
use crate::std::application::{Application, Error};
use crate::std::random::Random;
use crate::system::std::frame::ColouredElement;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i8,
    y: i8,
    dir: Direction,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Scored,
    Lost,
    Exited,
    None,
}

pub struct Game {
    snakes: Vec<Snake>,
    pois: Vec<Point>,
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
        Screen::application_mode();

        // render the initial state of the screen.
        self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;

        // make the first poi

        self.snakes.borrow_mut().push(Snake::player(0));

        for i in 0..5 {
            self.new_poi();
            self.snakes.borrow_mut().push(Snake::ai(i + 1));
        }

        // run the game
        self.gameloop().await?;

        // return to the terminal
        Screen::terminal_mode();
        Ok(())
    }
}

impl Game {

    async fn gameloop(&mut self) -> Result<(), Error> { // main gameloop
        let mut all_points: Vec<Point>;

        'gameloop: loop {

            time::wait(0.1);

            let mut points: Vec<Point>;
            let length = self.snakes.len();
            let mut status = Vec::new();

            for i in 0..length {
                let points: Vec<Point> = self.snakes.clone().into_iter().map(|s| s.tail).flatten().collect();
                let res = self.snakes[i].next(&self.pois, &points);
                status.push(res);



            }


            let res = self.snakes.iter_mut().map(|s| {

                let points: Vec<Point> = self.snakes.clone().into_iter().map(|s| s.tail).flatten().collect();
                s.next(&self.pois, &points)
            }).collect::<Vec<Status>>();


            if res.contains(&Status::Lost) {
                self.render_end_screen().map_err(|_| Error::ApplicationError(String::from("failed to render end screen")))?;

                // loop triggers when game is lost
                loop {
                    match Stdin::keystroke().await {
                        'x' => break 'gameloop,
                        _ => continue,
                    }
                }
            } else if res.contains(&Status::Exited) {
                break 'gameloop;
            } else if res.contains(&Status::Scored) {
                self.score += 1;
            }

            self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
        };
        Ok(())
    }

    fn new_poi(&mut self) {
        self.pois.push(Point { x: Random::int(3, 76) as i8, y: Random::int(3, 21) as i8 });
    }

    fn replace_poi(&mut self, poi: &Point) {
        self.pois.remove(self.pois.iter().position(|p| p == poi).unwrap());
        self.new_poi();
    }

    fn render(&mut self) -> Result<(), ()> {
        let mut frame = vec![vec![ScreenChar::null(); 80]; 25];
        self.snakes.borrow().clone().into_iter().map(|s| s.tail).flatten().for_each(|p| {
            frame[p.y as usize][p.x as usize] = ScreenChar::new('@' as u8, ColorCode::new(Color::Cyan, Color::Black));
        });

        self.pois.iter().for_each(|poi| {
            frame[poi.y as usize][poi.x as usize] = ScreenChar::new('o' as u8, ColorCode::new(Color::Red, Color::Black));
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
    head: Point,
    tail: Vec<Point>,
    dir: Direction,
}

impl Snake {
    fn ai(id: usize) -> Self {
        Self {
            ai_controlled: true,
            head: Point { x: 1 + id as i8 * 2, y: 1 },
            tail: Vec::new(),
            dir: Direction::Up
        }
    }
    fn player(id: usize) -> Self {
        Self {
            ai_controlled: false,
            head: Point { x: 1 + id as i8 * 2, y: 1 },
            tail: Vec::new(),
            dir: Direction::Up,
        }
    }

    fn next(&mut self, points_of_interest: &Vec<Point>, tails: &Vec<Point>) -> Status {  // returns (lose_condition, scored)

        // uses pathing algorithm if ai else keyboard input if human
        if self.ai_controlled {
            self.dir = self.decide_dir();
        } else {
            if let Some(c) = Stdin::try_keystroke() {
                self.dir = match c {
                    'w' => Direction::Up,
                    'a' => Direction::Left,
                    's' => Direction::Down,
                    'd' => Direction::Right,
                    'x' => return Status::Exited,
                    _ => self.dir.clone(),
                }
            }
        }

        self.tail.push(self.head.clone());

        match self.dir {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }

        if self.lose_condition(tails) {
            if !self.ai_controlled {
                return Status::Lost;
            }
        }

        if points_of_interest.contains(&self.head) {
            if !self.ai_controlled {
                return Status::Scored;
            }
        } else {
            self.tail.remove(0);
        }

        Status::None
    }

    fn decide_dir(&mut self) -> Direction {
        unimplemented!() // implement a basic pathfinding or random movement algorithm
    }

    fn lose_condition(&mut self, tails: &Vec<Point>) -> bool { // where tails includes the tail of every other snake
        let p = self.head.clone();

        let snake_overlaps = tails.contains(&self.head); // checks if any part of the snake overlaps itself
        let out_of_bounds = p.x < 0 || p.y < 0 || p.x > 79 || p.y > 24; // checks if the snake goes out of bounds

        snake_overlaps || out_of_bounds
    }
}


fn round_up(n: f64) -> usize {
    (n + 0.99) as usize
}
fn round_down(n: f64) -> usize {
    n as usize
}
