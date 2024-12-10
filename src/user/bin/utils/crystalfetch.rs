use alloc::{boxed::Box, format, string::String, vec::Vec};
use async_trait::async_trait;

use crate::println;
use crate::std::{
    application::{Application, Error},
    io::{write, Color, Screen},
    os::OS,
};

const _CRYSTAL_LOGO: &str = "\n  
  $$$$$$\\                              $$\\             $$\\ $$$$$$\\  $$$$$$\\
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

    /$$$$$$$$ /$$   /$$  /$$$$$$   /$$$$$$                     /$$
   |_____ $$ | $$  / $$ /$$__  $$ /$$__  $$                  /$$$$
        /$$/ |  $$/ $$/| $$  \\ $$| $$  \\__/       /$$    /$$|_  $$
       /$$/   \\  $$$$/ | $$  | $$|  $$$$$$       |  $$  /$$/  | $$
      /$$/     >$$  $$ | $$  | $$|\\____  $$       \\  $$/$$/   | $$
     /$$/     /$$/\\  $$| $$/$$ $$ /$$  \\ $$        \\  $$$/    | $$
    /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$/         \\  $/    /$$$$$$
   |________/|__/  |__/ \\____ $$$ \\______/           \\_/    |______/
";
const _ZXQ5_LOGO_OLD: &str = "

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

        let logo_string = ZXQ5_LOGO;
        let info_string = format!(
            " [   OS      »  {}
 [   BUILD   »  {}
 [   Shell   »  CrySH
 [   Github  »  https://github.com/FantasyPvP/CrystalOS
 [   Author  »  ZXQ5",
            os, version
        );

        // write to output
        let spacer =
            "\n".repeat(25 - logo_string.lines().count() - 4 - info_string.lines().count());
        // write values to console
        write(format_args!("{}", logo_string), (Color::Cyan, Color::Black));
        println!("\n");
        println!("{}", info_string);
        println!("{}", spacer);

        Ok(())
    }
}
