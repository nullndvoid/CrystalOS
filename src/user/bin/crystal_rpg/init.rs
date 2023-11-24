use async_trait::async_trait;
use super::{
    engine::{Choice, Event, eventcheck},
    entity::{Enemy, Entity, EntityObject},
    player::Player,
};

use alloc::{borrow::ToOwned, format, string::{String, ToString}, vec::Vec, boxed::Box};

use crate::{
    std::{
        io::{self, println, serial_println},
        random,
    },
    std::application::{
        Application,
        Error,
    },
};
use crate::std::io::{KeyStroke, Stdin};

pub struct GameLoop;


#[async_trait]
impl Application for GameLoop {
    fn new() -> Self {
        Self {}
    }
    async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

        let mut username: String = io::Stdin::readline().await;
        username = username.trim().to_string();

        let mut player = Player::new(username);

        let mut enemy = Enemy::new();

        for _ in 0..30 {
            match (eventcheck(player.attack_entity(&mut EntityObject::Enemy(&mut enemy)))) {
                Choice::A(result) => {
                    println!("{}", result);
                },
                Choice::B(event) => {
                    println!("{}", event);
                    match event {
                        Event::PlayerKilled => {
                            println!(" [!] {} was slain by Enemy\n\n[ You lost! ]", player.username);
                            break;
                        }
                        Event::EntityKilled(entity) => {
                            println!("\n [!] Enemy was slain by {}\n\n [ You won! ]", player.username);
                            break;
                        }
                    }
                }
            }
            println!("{}", eventcheck(enemy.attack_entity(&mut EntityObject::Player(&mut player))));
            println!("[{}\n[{}", player, enemy);
        }
 
        // FRAMEGEN.lock().render_frame();

       
        let string = String::from(format!(
"┌────────────────────────────┐
│   {}                        
│   {} / {}                     
└────────────────────────────┘"
        , player.username, player.health_points, player.max_health_points));
        // let mut healthbar = Element::from_str(string);
        // healthbar.render((1, 1));
        //
        // let new2 = String::from("[an element]");
        // let mut new = Element::from_str(new2);
        //
        //
        // new.render((10, 10));
        // new.render((10, 15));
        // new.render((5, 20));
        // new.render((34, 16));
        //
        //
        // FRAMEGEN.lock().render_frame();
        //
        // let fr = FRAMEGEN.lock().get_frame().to_owned();
        // serial_println!("{}", {
        //     let mut string = String::new();
        //     for row in fr {
        //         let mut r = String::new();
        //         for col in row {
        //             r.push(col.character as char);
        //         }
        //         string.push_str(&r);
        //         string.push('\n')
        //     };
        //     string
        // });


        loop {
            if let KeyStroke::Char(c) = Stdin::keystroke().await {
                println!("{}", c)
            }
        }

        Ok(())
    }
}

fn random() -> u64 {
    let r = random::Random::int(0, 125) as u64;
    r
}















