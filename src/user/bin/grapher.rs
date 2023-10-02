use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use async_trait::async_trait;
use crate::println;
use crate::shell::command_handler;
use crate::std::application::{Application, Error};
use crate::std::frame::Element;
use crate::std::io::{Screen, Stdin};

pub struct Grapher {
    points: Vec<Point>,
}

struct Point {
    x: i16,
    y: i16,
}

#[async_trait]
impl Application for Grapher {
    fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
    async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
        let mut equation: String = args.into_iter().collect();
        use super::calc;

        for x in -40..40 {
            let new_eq = equation.chars().map(|c| {
                if c == 'x' { x.to_string() } else { c.to_string() }
            }).collect::<String>();

            let y = calc::calc_outer(new_eq).map_err(|_| Error::ApplicationError(String::from("failed to calculate")))?;

            self.points.push(Point {
                x,
                y: y as i16,
            })
        };

        Screen::application_mode();
        self.render();

        loop {
            match Stdin::keystroke().await {
                'x' => break,
                _ => continue,
            }
        }

        Screen::terminal_mode();
        Ok(())
    }
}

impl Grapher {
    fn render(&self) {
        let mut frame: Vec<Vec<char>> = vec![vec![' '; 80]; 25];

        self.points.iter().filter(|i| i.x >= -40 && i.x < 40 && i.y >= -12 && i.y <= 12).for_each(|i| {
            let offset_x = i.x + 40;
            let offset_y = i.y + 12;
            //println!("{} {}", i.x, i.y);
            frame[24-offset_y as usize][offset_x as usize] = '*';
        });

        let mut elem = Element::generate(frame, (80, 25));
        elem.render((0, 0));
    }
}