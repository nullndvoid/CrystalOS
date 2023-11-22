use async_trait::async_trait;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use alloc::{boxed::Box, string::{String, ToString}, vec, vec::Vec};
use vga::writers::{GraphicsWriter, PrimitiveDrawing};

use crate::{print, printerr, println, serial_println, std, std::application::{Application, Error}, user::bin::*};
use crate::kernel::render::ScreenChar;
use crate::std::frame::ColouredElement;
use crate::std::io::{Color, write, Screen, Stdin, Serial};
use crate::std::random::Random;
use crate::user::bin::gigachad_detector::GigachadDetector;
use crate::user::bin::grapher::Grapher;
use crate::user::lib::libgui;
use crate::user::lib::libgui::{
    cg_core::{Frame, Position, Dimensions, CgComponent},
    cg_widgets::{CgTextBox, CgContainer},
};
use crate::user::lib::libgui::cg_widgets::CgLabel;

lazy_static! {
    pub static ref CMD: Mutex<CommandHandler> = Mutex::new(CommandHandler::new());
}

/// boilerplate function
/// may provide other interfacing options later on idk.
pub async fn command_handler() {
    eventloop().await;
}

/// this function starts the shell running, the function will loop repeatedly until the command to shutdown
/// TODO: implement shutdown command
pub async fn eventloop() {
    println!("running!");

    let mut fetch = crystalfetch::CrystalFetch::new();
    let string = String::from(" ");
    let mut vec: Vec<String> = Vec::new();
    vec.push(string);
    fetch.run(vec).await.unwrap();

    CMD.lock().prompt();

    loop {
        let string = crate::std::io::Stdin::readline().await;
        CMD.lock().current.push_str(&string);
        match exec().await {
            Ok(_) => {
                ();
            }
            Err(e) => {
                handle_error(e);
            }
        };
        CMD.lock().prompt();
    }
}

fn handle_error(e: Error) {
    match e {
        Error::EmptyCommand => {
            printerr!("empty command");
        },
        Error::UnknownCommand(cmd_str) => {
            printerr!("unknown command: '{}'", cmd_str);
        },
        Error::ApplicationError(e) => {
            printerr!("application returned error:\n{}", e);
        },
        Error::CommandFailed(e) => {
            printerr!("command failed:\n{}", e);
        },
    }
}

async fn exec() -> Result<(), Error> {
    let mut current = CMD.lock().current.clone();

    CMD.lock().history.history.push(current.clone());

    current.pop();
    CMD.lock().current = String::new();

    let (cmd, args) = match CommandHandler::parse_args(current) {
        Ok((cmd, args)) => (cmd, args),
        Err(_) => {
            return Err(Error::EmptyCommand);
        }
    };

    match cmd.as_str() {
        "calculate" | "calc" | "solve" => {
            let mut cmd = calc::Calculator::new();
            cmd.run(args).await?;
        }

        "rickroll" => {
            let mut cmd = rickroll::Rickroll::new();
            cmd.run(args).await?;
        }

        "crystalfetch" => {
            let mut cmd = crystalfetch::CrystalFetch::new();
            cmd.run(args).await?;
        }
        "tasks" => {
            let mut cmd = tasks::Tasks::new();
            cmd.run(args).await?;
        }
        "play" => {
            let mut gameloop = crystal_rpg::init::GameLoop::new();
            gameloop.run(args).await?;
        }
        "VGA" => {
            use vga::colors::Color16;
            use vga::writers::{GraphicsWriter, Graphics640x480x16};

            let mode = Graphics640x480x16::new();
            mode.set_mode();
            mode.clear_screen(Color16::Black);
            mode.draw_line((80, 60), (80, 420), Color16::Cyan);
        }
        "graph" => {
            let mut grapher = Grapher::new();
            grapher.run(args).await?;
        }
        "snake" => {
            let mut game = snake::Game::new();
            game.run(args).await?;
        }
        "serial" => {
            let c = Serial::reply_char('e');
            println!("{}", c);
        }
        "gameoflife" => {
            let mut game = gameoflife::GameOfLife::new();
            game.run(Vec::new()).await?;
        }
        "tetris" => {
            let mut game = tetris::TetrisEngine::new();
            game.run(Vec::new()).await?;
        }

        "gigachad?" => {
            let mut gigachad_detector = GigachadDetector::new();
            gigachad_detector.run(args).await?;
        }

        "wait" => {
            use std::time::wait;

            if args.len() != 1 {
                return Err(Error::CommandFailed("exactly one argument must be provided".to_string()))
            }
            if let Ok(time) = args[0].parse::<u64>() {
                wait(time as f64);
                println!("waited for {}s", time);
            }
        }

        // direct OS functions (not applications)
        "echo" => {
            println!(
                "Crystal: '{}'",
                args.into_iter()
                    .map(|mut s| {
                        s.push_str(" ");
                        s
                    })
                    .collect::<String>()
            )
        }
        "clear" => {
            Screen::clear();
            // not sure why this code was here but leaving it in case weird bugs happen so i remember to add it back if so
            //interrupts::without_interrupts(|| {});
        }
        "switch" => {
            Screen::switch();
        }
        "time" => {
            use crate::std::time::timer;
            timer();
        }
		"test_features" => {
            std::io::Screen::application_mode();

            serial_println!("OK");

            let textbox = CgTextBox::new(
                String::from("i'd just like to interject for a moment"),
                String::from("I'd just like to interject for a moment. What you're refering to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called Linux, and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use. Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called Linux distributions are really distributions of GNU/Linux!"),
                Position::new(2, 2),
                Dimensions::new(40, 12),
                true,
            );

            let textbox2 = CgTextBox::new(
                String::from("i'd just like to interject for a moment"),
                String::from("I'd just like to interject for a moment. What you're refering to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called Linux, and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use. Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called Linux distributions are really distributions of GNU/Linux!"),
                Position::new(10, 5),
                Dimensions::new(40, 12),
                true,
            );


            let mut label = CgLabel::new(
                String::from("i'd just like to interject for a moment"),
                Position::new(5, 16),
                15,
            );

            let mut container = CgContainer::new(
                Position::new(2, 2),
                Dimensions::new(65, 20),
                true,
            );

            container.elements.push(Box::new(textbox));
            container.elements.push(Box::new(textbox2));
            container.elements.push(Box::new(label));

            let res = if let Ok(frame) = container.render() {
                frame
            } else {
                return Err(Error::CommandFailed("failed to render frame".to_string()))
            };

            let mut elem = ColouredElement::generate(
                res.render_screen_char(),
                (res.dimensions().x as u8, res.dimensions().y as u8),
            );

            elem.render((0, 0)).unwrap();

            //std::io::Screen::terminal_mode();

		}
        _ => {
            return Err(Error::UnknownCommand(
                cmd
            ))
        }
    };

    Ok(())
}

pub struct CommandHandler {
    current: String,
    history: CmdHistory,
}

impl CommandHandler {
    pub fn new() -> Self {
        let handler = Self {
            current: String::new(),
            history: CmdHistory {
                history: Vec::new(),
            },
        };
        handler
    }

    pub fn parse_args(command: String) -> Result<(String, Vec<String>), String> {
        let temp = command.split(" ").collect::<Vec<&str>>();
        let mut args: Vec<String> = Vec::new();
        for arg in temp {
            match arg {
                "" => {}
                x => args.push(x.to_string()),
            }
        }
        let cmd: String;
        if args.len() > 0 {
            cmd = args[0].clone();
            args.remove(0);
        } else {
            return Err("command was empty.".to_string());
        };
        Ok((cmd, args))
    }

    // this function is activated every time the user presses a key on the keyboard
    // it accesses the queue of keys (a static ref in src/tasks/keyboard.rs)

    // displays a text prompt for the user to type into.
    // this is a separate function so that it can be developed as necessary later on
    // TODO: coloured prompt

    pub fn prompt(&self) {
        write(format_args!("\n Crystal> "), (Color::Cyan, Color::Black));
    }
}

struct CmdHistory {
    history: Vec<String>,
}
