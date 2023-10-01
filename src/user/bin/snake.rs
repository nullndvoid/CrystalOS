use alloc::string::String;
use alloc::{format, vec, vec::Vec, boxed::Box};
use async_trait::async_trait;
use crate::std::io::{Color, Screen};
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

pub struct Game {
    snake: SegQueue<Point>,
    head: Point,
    poi: Point,
    score: u8
}

#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Self {
            snake: SegQueue::new(),
            head: Point { x: 5, y: 5 },
            poi: Point { x: 0, y: 0 },
            score: 0
        }
    }

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        Screen::application_mode();

        (0..=2).for_each(|x| {
            self.snake.push(Point { x, y: 5 });
        });
        self.head = Point { x: 2, y: 5 };
        self.new_poi();

        'gameloop: loop {
            let chr = KEYBOARD.lock().get_keystroke().await;
            match chr {
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

        Screen::terminal_mode();
        Ok(())
    }
}

impl Game {
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

fn round_up(n: f64) -> usize {
    (n + 0.99) as usize
}
fn round_down(n: f64) -> usize {
    n as usize
}
