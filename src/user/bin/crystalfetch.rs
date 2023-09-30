use async_trait::async_trait;
use alloc::{boxed::Box, format, string::String, vec::Vec};
use log::info;

use crate::{std::os::OS, std::io::{Color, write, Screen}, println, std::application::{
	Application,
	Error,
}, std};

const CRYSTAL_LOGO: &str =
" $$$$$$\\                              $$\\             $$\\ $$$$$$\\  $$$$$$\\
$$  __$$\\                             $$ |            $$ $$  __$$\\$$  __$$\\
$$ /  \\__|$$$$$$\\ $$\\   $$\\ $$$$$$$\\$$$$$$\\   $$$$$$\\ $$ $$ /  $$ $$ /  \\__|
$$ |     $$  __$$\\$$ |  $$ $$  _____\\_$$  _|  \\____$$\\$$ $$ |  $$ \\$$$$$$\\
$$ |     $$ |  \\__$$ |  $$ \\$$$$$$\\   $$ |    $$$$$$$ $$ $$ |  $$ |\\____$$\\
$$ |  $$\\$$ |     $$ |  $$ |\\____$$\\  $$ |$$\\$$  __$$ $$ $$ |  $$ $$\\   $$ |
\\$$$$$$  $$ |     \\$$$$$$$ $$$$$$$  | \\$$$$  \\$$$$$$$ $$ |$$$$$$  \\$$$$$$  |
 \\______/\\__|      \\____$$ \\_______/   \\$$$$$$\\_______\\__|\\______/ \\______/
                  $$\\   $$ |           $$  __$$\\
                  \\$$$$$$  |  $$\\    $$\\__/  $$ |
                   \\______/   \\$$\\  $$  $$$$$$  |
                               \\$$\\$$  $$  ____/
                                \\$$$  /$$ |
                                 \\$  / $$$$$$$$\\
                                  \\_/  \\________|                           ";

const ZXQ5_LOGO: &str = "

    /$$$$$$$$ /$$   /$$  /$$$$$$   /$$$$$$   /$$$$$$                     /$$
   |_____ $$ | $$  / $$ /$$__  $$ /$$__  $$ /$$__  $$                  /$$$$
        /$$/ |  $$/ $$/| $$  \\ $$| $$  \\ $$| $$  \\__/       /$$    /$$|_  $$
       /$$/   \\  $$$$/ | $$  | $$| $$  | $$|  $$$$$$       |  $$  /$$/  | $$
      /$$/     >$$  $$ | $$  | $$| $$  | $$ \\____  $$       \\  $$/$$/   | $$
     /$$/     /$$/\\  $$| $$/$$ $$| $$  | $$ /$$  \\ $$        \\  $$$/    | $$
    /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$/|  $$$$$$/         \\  $/    /$$$$$$
   |________/|__/  |__/ \\____ $$$ \\______/  \\______/           \\_/    |______/
";
pub struct CrystalFetch {}

#[async_trait]
impl Application for CrystalFetch {
	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

		let os = OS.lock().os.clone();
		let version = OS.lock().version.clone();

		Screen::clear();

		let logo_string = CRYSTAL_LOGO;
		let info_string = format!(
" [   OS      »  {}
 [   BUILD   »  {}
 [   Shell   »  CrySH
 [   Github  »  https://github.com/FantasyPvP/CrystalOS-Restructured
 [   Author  »  FantasyPvP / ZXQ5", os, version);

		// write to output
		let spacer = "\n".repeat(24 - logo_string.lines().count() - 4 - info_string.lines().count());
		// write values to console
		write(format_args!("{}", logo_string), (Color::Cyan, Color::Black));
		println!("\n\n");
		println!("{}", info_string);
		println!("{}", spacer);

		Ok(())
	}

}
