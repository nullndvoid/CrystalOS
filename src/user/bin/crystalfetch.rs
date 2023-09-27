use async_trait::async_trait;
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{std::os::OS, std::io::{Color, write, clear}, println, std::application::{
	Application,
	Error,
}, std};


pub struct CrystalFetch {}

#[async_trait]
impl Application for CrystalFetch {

	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

		let os = OS.lock().os.clone();
		let version = OS.lock().version.clone();

		clear();

		write(format_args!("
    /$$$$$$$$ /$$   /$$  /$$$$$$   /$$$$$$   /$$$$$$                     /$$
   |_____ $$ | $$  / $$ /$$__  $$ /$$__  $$ /$$__  $$                  /$$$$
        /$$/ |  $$/ $$/| $$  \\ $$| $$  \\ $$| $$  \\__/       /$$    /$$|_  $$
       /$$/   \\  $$$$/ | $$  | $$| $$  | $$|  $$$$$$       |  $$  /$$/  | $$
      /$$/     >$$  $$ | $$  | $$| $$  | $$ \\____  $$       \\  $$/$$/   | $$
     /$$/     /$$/\\  $$| $$/$$ $$| $$  | $$ /$$  \\ $$        \\  $$$/    | $$
    /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$/|  $$$$$$/         \\  $/    /$$$$$$
   |________/|__/  |__/ \\____ $$$ \\______/  \\______/           \\_/    |______/
"), (Color::Cyan, Color::Black));
		println!("


 [   OS      »  {}
 [   BUILD   »  {}
 [   Shell   »  CrySH
 [   Github  »  https://github.com/FantasyPvP/CrystalOS-Restructured
 [   Author  »  FantasyPvP / ZXQ5




", os, version);
		Ok(())
	}

}
