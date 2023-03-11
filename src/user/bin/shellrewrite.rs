
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
    kernel::tasks::{Task, Executor},
    applications::*,
    std::application::{Application, Error},
    std::io::{stdin, print, println},
    user::bin::*,
};



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




/// initialises a global tasks struct, this can be accessed from anywhere in the program;
lazy_static! {
    pub static ref TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
}


/// starts the shell
/// this function should be directly called by main.rs or by an init system
pub fn init_sh(args: Option<Vec<String>>) -> Result<(), String> {
    let mut executor = Executor::new();

    loop {
        executor.spawn(Task::new(next()));
                
        let tasks = TASKS.lock();
        while tasks.len() > 0 {
            let next = tasks[0].clone();
            tasks.remove(0);
            executor.spawn(next);
        }
        drop(tasks);

        executor.run();
    }
    
    Ok(())
}

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

    
fn run_binary(binary: Application) -> Result<Vec<String>, String> {
    binary.run()
    Ok(Vec::<String::new()>)
}

async fn next() {
    let command: String = stdin();
    let parsed = match parse_args(command) {
        Ok(x) -> x
        Err(e) -> {
            println!("Error Parsing Command: Invalid Syntax")
        }
    }
    // tokens will eventually be parsed here

    /*
    - PARSER
    - this will allow the use of more complex commands later down the line
    */
    TASKS.lock().push(Task::new(cmd));
}
















