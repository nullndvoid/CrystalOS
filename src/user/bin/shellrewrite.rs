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
    applications::*,
    std::application::{Application, Error},
    std::io::{print, println},
    user::bin::*,
};

lazy_static! {
    pub static ref CMD: Mutex<Cmd> = Mutex::new(Cmd::new());
}
struct Cmd {}
impl Cmd {
    fn new() -> Self {
        Self {}
    }
}

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

pub fn init_sh() -> Result<(), String> {
    Ok(())
}
