use alloc::string::String;
use alloc::{format, vec, vec::Vec, boxed::Box};
use async_trait::async_trait;
use crate::std::io::{Color, Screen, Stdin};
use crate::std::time;
use crate::kernel::tasks::keyboard::KEYBOARD;
use crossbeam_queue::SegQueue;
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

enum Status {
    Scored,
    Lost,
    Exited,
    None,
}

pub struct Game {
    snake: SegQueue<Point>,
    head: Point,
    poi: Point,
    mv: char,
    score: u8,
    hardmode: bool,
}


snake.rs
#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Self {
            snake: SegQueue::new(),
            head: Point { x: 5, y: 5 },
            poi: Point { x: 0, y: 0 },
            mv: ' ',
            score: 0,
            hardmode: false,
        }
    }

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        Screen::application_mode();
        let clone = self.clone_snake();

        // render the initial state of the screen.
        self.render(clone).map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;


        (5..=7).for_each(|x| {
            self.snake.push(Point { x, y: 5 });
        });
        self.head = Point { x: 7, y: 5 };
        self.new_poi();

        self.gameloop().await?;

        Screen::terminal_mode();
        Ok(())
    }
}

impl Game {

    async fn gameloop(&mut self) -> Result<(), Error> { // main gameloop
        'gameloop: loop {

            time::wait(0.1);

            if let Some(c) = Stdin::try_keystroke() {
                self.mv = c;
            }

            //self.mv = Stdin::keystroke().await;

            match self.mv {
                'w' => self.head.y -= 1,
                'a' => self.head.x -= 1,
                's' => self.head.y += 1,
                'd' => self.head.x += 1,
                'x' => break,
                _ => continue,
            }

            self.snake.push(Point { x: self.head.x, y: self.head.y }); // new head added

            if self.head == self.poi {
                self.new_poi();
                self.score += 1
            } else {
                self.snake.pop().unwrap(); // tail removed if score does not increase
            }

            if self.lose_condition() {
                self.render_end_screen().map_err(|_| Error::ApplicationError(String::from("failed to render game over screen")))?;
                while let chr = KEYBOARD.lock().get_keystroke().await {
                    match chr {
                        'x' => break 'gameloop,
                        _ => continue,
                    }
                }
            }

            let clone = self.clone_snake();
            self.render(clone).map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
        };
        Ok(())
    }

    fn new_poi(&mut self) {
        self.poi = Point { x: Random::int(3, 76) as i8, y: Random::int(3, 21) as i8 }
    }

    fn render(&mut self, snake: Vec<Point>) -> Result<(), ()> {
        let mut frame = vec![vec![ScreenChar::null(); 80]; 25];
        snake.into_iter().for_each(|p| {
            frame[p.y as usize][p.x as usize] = ScreenChar::new('@' as u8, ColorCode::new(Color::Cyan, Color::Black));
        });

        frame[self.poi.y as usize][self.poi.x as usize] = ScreenChar::new('o' as u8, ColorCode::new(Color::Red, Color::Black));
        let literal = format!("snake go brr score:  {}", self.score);
        let msg = Game::centre_text(80, literal);
        frame[1] = msg.chars().map(|c| ScreenChar::new(c as u8, ColorCode::new(Color::LightGreen, Color::Black))).collect();

        let mut elem = ColouredElement::generate(frame, (80, 25));
        elem.render((0,0))
    }

    fn lose_condition(&mut self) -> bool {
        let cloned = self.clone_snake();
        let snake_overlaps = (1..cloned.len()).any(|i| cloned[i..].contains(&cloned[i - 1])); // checks if any part of the snake overlaps itself
        let out_of_bounds = cloned.iter().filter(|p| p.x < 0 || p.y < 0 || p.x > 79 || p.y > 24).count() > 0; // checks if the snake goes out of bounds

        snake_overlaps || out_of_bounds
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

    fn clone_snake(&mut self) -> Vec<Point> {
        let mut cloned= Vec::new();
        let mut snake = SegQueue::new();
        while !self.snake.is_empty() {
            let item = self.snake.pop().unwrap();
            cloned.push(item.clone());
            snake.push(item);
        }
        self.snake = snake;
        cloned
    }
}













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
            return Status::Lost;
        }

        if points_of_interest.contains(&self.head) {
            Status::Scored
        } else {
            self.tail.remove(0);
            Status::None
        }
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
