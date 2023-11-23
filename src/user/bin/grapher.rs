use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use alloc::boxed::Box;
use async_trait::async_trait;
use crate::{println, serial_println};
use crate::shell::command_handler;
use crate::std::application::{Application, Error};
use crate::std::frame::{self, Frame, Position, Dimensions, ColouredChar};
use crate::std::io::{Screen, Stdin};

const OFFSET_X: i64 = 40;
const OFFSET_Y: i64 = 12;

pub struct Grapher {
    points: Vec<PointF64>,
    frame: Frame,
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
            frame: Frame::new(Position::new(0, 0), Dimensions::new(80, 25)).unwrap()
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

        Screen::Application.set_mode().map_err(|_| Error::ApplicationError(String::from("failed to set application mode")))?;
        self.display();

        loop {
            match Stdin::keystroke().await {
                'x' => break,
                _ => continue,
            }
        }

        Screen::Terminal.set_mode().map_err(|_| Error::ApplicationError(String::from("failed to set terminal mode")))?;
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

        // serial_println!("{} {}", 24-offset_y as usize, offset_x as usize);

        self.frame.write(Position::new(offset_x as usize, 24-offset_y as usize), ColouredChar::new('*'));
    }


    fn display(&mut self) {
        self.frame.render_to_screen().unwrap();
    }
}