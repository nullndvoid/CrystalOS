// importing libraries
use async_trait::async_trait;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    kernel::tasks::{executor::Executor, Task},
    std::application::{Application, Error},
    std::io::{print, println, Stdin, Screen},
    user::bin::*,
};

use super::*;

//  [ CRYSTAL SHELL ]
//  the purpose of this module is to provide a basic unix shell like experience for the user
//  to interact with the OS
//  this is a rewrite of my original shell.
//  this shell should support:
//  - browsing the virtual filesystem
//  - executing programs
//  - basic arithmetic
//  - chained execution ( multiple commands linked together) eg: '5 + 5 | echo' which calculates
//    the result of 5 + 5 and then sends the result to an echo command which prints it to console


/// starts the shell
/// this function should be directly called by main.rs or by an init system

fn new_function() {

}


pub fn userspace() -> Result<(), String> {
    let mut executor = Executor::new();

    //
    // executor.spawn(Task::new(new_function()));
    // loop {
    //     executor.try_run()
    // }
    Ok(())
}

// struct Shell {}
//
// impl Application for Shell {
//     fn new() -> Shell {
//         Shell {}
//     }
//     async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
//         Ok(())
//     }
// }




fn parse_args(command: String) -> Result<(String, Vec<String>), String> {
    let mut args: Vec<String> = Vec::new();

    for arg in command.split(" ").collect::<Vec<&str>>() {
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

//fn run_binary(binary: dyn Application) -> Result<Vec<String>, String> {
//    binary.run();
//    Ok(Vec::<String::new()>)
//}

