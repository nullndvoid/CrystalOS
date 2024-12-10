// // importing libraries
// use async_trait::async_trait;
// use lazy_static::lazy_static;
// use spin::Mutex;
// use x86_64::instructions::interrupts;

// use alloc::{boxed::Box, string::{String, ToString}, vec, vec::Vec};

// use crate::{
//     kernel::tasks::{executor::Executor, Task},
//     std::application::{Application, Error},
//     std::io::{print, println, Stdin, Screen},
//     user::bin::*,
// };
// use crate::std::io::{Color, write};
// use crate::user::bin::gigachad_detector::GigachadDetector;

// use super::*;

// //  [ CRYSTAL SHELL ]
// //  the purpose of this module is to provide a basic unix shell like experience for the user
// //  to interact with the OS
// //  this is a rewrite of my original shell.
// //  this shell should support:
// //  - browsing the virtual filesystem
// //  - executing programs
// //  - basic arithmetic
// //  - chained execution ( multiple commands linked together) eg: '5 + 5 | echo' which calculates
// //    the result of 5 + 5 and then sends the result to an echo command which prints it to console

// /// starts the shell
// /// this function should be directly called by main.rs or by an init system

// fn run_task(task_name: String, args: Vec<String>) -> Result<(), String> {
//     Ok(())
// }

// pub async fn userspace() -> Result<(), String> {
//     let mut executor = Executor::new();

//     let mut shell = Shell::new();
//     shell.run(vec![]).await.unwrap();

//     Ok(())
// }

// struct Shell {
//     history: Vec<String>,
// }

// #[async_trait]
// impl Application for Shell {
//     fn new() -> Shell {
//         Shell {
//             history: Vec::new(),
//         }
//     }
//     async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
//         loop {
//             self.prompt();
//             let input = Stdin::readline().await;
//             let (cmd, args) = self.parse_args(input).unwrap();
//             self.run_cmd(cmd, args).await.unwrap();
//         }
//     }
// }

// impl Shell {
//     fn prompt(&mut self) {
//         write(format_args!("\n Crystal> "), (Color::Cyan, Color::Black));
//     }

//     // fn exec<R, T: Fn() -> R>(command: T) -> Result<R, Error> { // this command runs when a shell command is executed
//     //     Ok(command())
//     // }
//     async fn run_cmd(&mut self, cmd: String, args: Vec<String>) -> Result<(), Error> {
//         match cmd.as_str() {
//             "calculate" | "calc" | "solve" => {
//                 let mut cmd = calc::Calculator::new();
//                 cmd.run(args).await?;
//             }
//             "rickroll" => {
//                 let mut cmd = rickroll::Rickroll::new();
//                 cmd.run(args).await?;
//             }
//             "crystalfetch" => {
//                 let mut cmd = crystalfetch::CrystalFetch::new();
//                 cmd.run(args).await?;
//             }
//             "tasks" => {
//                 let mut cmd = tasks::Tasks::new();
//                 cmd.run(args).await?;
//             }
//             "play" => {
//                 let mut gameloop = crystal_rpg::init::GameLoop::new();
//                 gameloop.run(args).await?;
//             }
//             "echo" => {
//                 println!(
//                     "Crystal: '{}'",
//                     " ".join(args)
//                 )
//             }
//             "clear" => {
//                 Screen::clear();
//             }
//             "print" => {
//                 use crate::std::os::OS;
//                 let x: String = OS.lock().version.clone();
//                 println!("{}", x);
//             }
//             "snake" => {
//                 let mut game = snake::Game::new();
//                 game.run(Vec::new()).await?;
//             }
//             "gigachad?" => {
//                 let mut gigachad_detector = GigachadDetector::new();
//                 gigachad_detector.run(args).await?;
//             }
//             "test_features" => {
//                 use crate::std::random::Random;
//                 println!("{}", Random::int(0, 10));
//             }
//             _ => {
//                 return Err(Error::UnknownCommand(
//                     "command not yet implemented".to_string(),
//                 ))
//             }
//         }
//         Ok(())
//     }
//     fn parse_args(&self, command: String) -> Result<(String, Vec<String>), String> {
//         let mut args: Vec<String> = Vec::new();

//         for arg in command.split(" ").collect::<Vec<&str>>() {
//             match arg {
//                 "" => {}
//                 x => args.push(x.to_string()),
//             }
//         }

//         let cmd: String;
//         if args.len() > 0 {
//             cmd = args[0].clone();
//             args.remove(0);
//         }
//         else {
//             return Err("command was empty.".to_string());
//         };

//         Ok((cmd, args))
//     }
// }
