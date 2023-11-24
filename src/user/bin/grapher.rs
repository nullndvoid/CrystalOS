use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use alloc::boxed::Box;
use async_trait::async_trait;
use crate::{println, serial_println};
use crate::kernel::render::{ColorCode, RenderError};
use crate::shell::command_handler;
use crate::std::application::{Application, Error};
use crate::std::frame::{self, Frame, Position, Dimensions, ColouredChar};
use crate::std::io::{Color, KeyStroke, Screen, Stdin};
use crate::user::lib::libgui::cg_core::{CgComponent, CgTextEdit};
use crate::user::lib::libgui::cg_inputs::CgLineEdit;
use crate::user::lib::libgui::cg_widgets::CgContainer;
use super::calc;

const OFFSET_X: i64 = 39;
const OFFSET_Y: i64 = 10;

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
            frame: Frame::new(Position::new(1, 1), Dimensions::new(78, 22)).unwrap()
        }
    }
    async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
        Screen::Application.set_mode().map_err(|_| Error::ApplicationError(String::from("failed to set application mode")))?;

        self.frame.frame = vec![vec![ColouredChar::coloured(' ', ColorCode::new(Color::White, Color::DarkGray)); self.frame.dimensions.x]; self.frame.dimensions.y];

        if args.len() > 0 {
            let equation: String = args.into_iter().collect();
            self.graph_equation(equation);

            if let Ok(frame) = self.render() {
                frame.write_to_screen().map_err(|_| Error::ApplicationError(String::from("failed to write to screen")))?;
            }

            loop {
                match Stdin::keystroke().await {
                    KeyStroke::Char('x') => break,
                    _ => continue,
                }
            }

            Screen::Terminal.set_mode().map_err(|_| Error::ApplicationError(String::from("failed to set terminal mode")))?;
            return Ok(());
        }
        else {
            let mut entry_box = CgLineEdit::new(
                Position::new(1, 23),
                78,
                String::from("function >")
            );

            let mut commandresult = String::new();

            while let c = Stdin::keystroke().await {
                let mut container = CgContainer::new(
                    Position::new(0, 0),
                    Dimensions::new(80, 25),
                    true,
                );

                match c {
                    KeyStroke::Char('\n') => {
                        commandresult = entry_box.text.iter().collect();
                        entry_box.clear();
                    },
                    KeyStroke::Char(Stdin::BACKSPACE) => {
                        serial_println!("backspace");
                        entry_box.backspace()
                    },
                    KeyStroke::Char(c) => entry_box.write_char(c),
                    KeyStroke::Left => entry_box.move_cursor(false),
                    KeyStroke::Right => entry_box.move_cursor(true),
                    KeyStroke::Alt => break,
                    _ => {}
                }

                if commandresult.len() > 0 {
                    let equation = commandresult.chars().take(40).collect();
                    self.graph_equation(equation);
                    commandresult.clear();
                }

                container.insert(Box::new(self));
                container.insert(Box::new(&entry_box));

                if let Ok(frame) = container.render() {
                    frame.write_to_screen().map_err(|_| Error::ApplicationError(String::from("failed to write to screen")))?;
                }
            }
        }

        Ok(())
    }
}

impl Grapher {

    fn graph_equation(&mut self, equation: String) {

        let cal = calc::Calculator::new();
        for x in -4000..4000 {
            let x = x as f64 / 100.0;

            let new_eq = equation.chars().map(|c| {
                if c == 'x' { format!("({})", x) } else { c.to_string() }
            }).collect::<String>();

            let fx = cal.calculate(new_eq);
            if let Ok(y) = fx {
                self.render_point(PointF64 {
                    x,
                    y,
                })
            }
        };
    }


    fn render_point(&mut self, point: PointF64) {
        let point = PointI64 {
            x: point.x as i64,
            y: point.y as i64,
        };

        if point.x < -39 || point.x >= 39 || point.y < -10 || point.y >= 12 {
            return;
        }

        let offset_x = point.x + OFFSET_X;
        let offset_y = point.y + OFFSET_Y;

        self.frame.write(Position::new(offset_x as usize, 22-offset_y as usize), ColouredChar::coloured('*', ColorCode::new(Color::White, Color::DarkGray)));
    }


    fn display(&mut self) {
        self.frame.write_to_screen().unwrap();
    }
}

impl CgComponent for Grapher {
    fn render(&self) -> Result<Frame, RenderError> {
        Ok(self.frame.clone())
    }
}












