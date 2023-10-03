use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use alloc::boxed::Box;
use async_trait::async_trait;
use crate::println;
use crate::shell::command_handler;
use crate::std::application::{Application, Error};
use crate::std::frame::Element;
use crate::std::io::{Screen, Stdin};

const OFFSET_X: i64 = 40;
const OFFSET_Y: i64 = 12;

pub struct Grapher {
    points: Vec<PointF64>,
    frame: Vec<Vec<char>>,
}

struct PointF64 {
    x: f64,
    y: f64,
}
struct PointI64 {
    x: i64,
    y: i64,
}


#[async_trait]
impl Application for Grapher {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            frame:  vec![vec![' '; 80]; 25],
        }
    }
    async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
        let mut equation: String = args.into_iter().collect();
        use super::calc;

        let cal = calc::Calculator::new();

        for x in -4000..4000 {
            let x = x as f64 / 100.0;

            let new_eq = equation.chars().map(|c| {
                if c == 'x' { format!("({})", x) } else { c.to_string() }
            }).collect::<String>();

            let fx = cal.calculate(new_eq).map_err(|_| Error::ApplicationError(String::from("failed to calculate")));

            if let Ok(y) = fx {
                self.render_point(PointF64 {
                    x,
                    y,
                })
            }
        };

        Screen::application_mode();
        self.display();

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

    fn render_point(&mut self, point: PointF64) {

        let point = PointI64 {
            x: point.x as i64,
            y: point.y as i64,
        };

        if point.x < -40 || point.x >= 40 || point.y < -12 || point.y >= 12 {
            return;
        }

        let offset_x = point.x + OFFSET_X;
        let offset_y = point.y + OFFSET_Y;

        self.frame[24-offset_y as usize][offset_x as usize] = '*';
    }


    fn display(&mut self) {
        let mut elem = Element::generate(self.frame.clone(), (80, 25));
        elem.render((0, 0));
    }
}