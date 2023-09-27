use async_trait::async_trait;
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    std::os::OS,
    std::io::{Color, write},
    println,
    std::application::{
        Application,
        Error,
    },
};

const GIGACHAD: &'static str = "fantasypvp";

pub struct GigachadDetector {}

#[async_trait]
impl Application for GigachadDetector {
    fn new() -> Self {
        Self {}
    }

    async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
        for arg in args {
            self.detect_gigachad_by_username(&arg)
        }
        Ok(())
    }
}

impl GigachadDetector {
    pub fn detect_gigachad_by_username(&self, username: &str) {
        if username == GIGACHAD {
            println!("{} is a gigachad B'YES", username);
            println!("
     /$$$$$$$$ /$$   /$$  /$$$$$$  /$$$$$$$
    |_____ $$ | $$  / $$ /$$__  $$| $$____/
         /$$/ |  $$/ $$/| $$  \\ $$| $$
        /$$/   \\  $$$$/ | $$  | $$| $$$$$$$
       /$$/     >$$  $$ | $$  | $$|_____  $$
      /$$/     /$$/\\  $$| $$/$$ $$ /$$  \\ $$
     /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$/
    |________/|__/  |__/ \\____ $$$ \\______/
                              \\__/
    ")
        } else {
            println!("{} is not a gigachad", username);
        }
    }
}