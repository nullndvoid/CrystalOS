// use alloc::{boxed::Box, string::String, vec::Vec};
// use core::cmp::min;
//
// struct Player {
//     username: String,
//     stats: EntityStats,
//
//     exp: u32,
//     level: u32,
//     skill_points: u32,
//     skills: Vec<Box<dyn Skill>>,
//
//     helmet: Option<Helmet>,
//     chestplate: Option<Chestplate>,
//     boots: Option<Boots>,
//
//     inventory: Vec<Item>,
// }
//
// struct EntityStats {
//     health: i32,
//     max_health: i32,
//     mana: i32,
//     max_mana: i32,
//     defence: i32,
// }
//
// impl Player {
//     fn new(username: String) -> Self {
//         Self {
//             username,
//             stats: EntityStats {
//                 health: 100,
//                 max_health: 100,
//                 mana: 100,
//                 max_mana: 100,
//                 defence: 0,
//             },
//             exp: 0,
//             level: 0,
//             skill_points: 0,
//             skills: Vec::new(),
//
//             helmet: None,
//             chestplate: None,
//             boots: None,
//             inventory: Vec::new(),
//         }
//     }
//
//     fn heal(&mut self, amount: i32) {
//         let max_health = self.max_health();
//
//         self.stats.health = min(self.stats.health + amount, max_health);
//     }
//
//     fn damage(&mut self, amount: i32) {
//         let hp = self.health_points();
//     }
//
//     fn inventory_contents_mut(&mut self) -> &mut Vec<Item> {
//         &mut self.inventory
//     }
//
//     fn max_health(&self) -> i32 {
//         let mut max_health = self.stats.max_health;
//
//         if let Some(helmet) = &self.helmet {
//             max_health += helmet.stats.health_bonus;
//         }
//         if let Some(chestplate) = &self.chestplate {
//             max_health += chestplate.stats.health_bonus;
//         }
//         if let Some(boots) = &self.boots {
//             max_health += boots.stats.health_bonus;
//         }
//
//         max_health
//     }
//
//     fn health_points(&self) -> i32 {
//         let mut hp = self.stats.health;
//         if let Some(helmet) = &self.helmet {
//             hp += helmet.stats.health_bonus;
//         }
//         if let Some(chestplate) = &self.chestplate {
//             hp += chestplate.stats.health_bonus;
//         }
//         if let Some(boots) = &self.boots {
//             hp += boots.stats.health_bonus;
//         }
//         hp
//     }
// }
//
// enum Item {
//     Helmet(Helmet),
//     Chestplate(Chestplate),
//     Boots(Boots),
//     Sword,
//     Potion,
// }
//
// struct Helmet {
//     name: &'static str,
//     lore: &'static str,
//     stats: ArmourStats,
// }
//
// struct Chestplate {
//     name: &'static str,
//     lore: &'static str,
//     stats: ArmourStats,
// }
//
// struct Boots {
//     name: &'static str,
//     lore: &'static str,
//     stats: ArmourStats,
// }
//
// struct ArmourStats {
//     durability: i32,
//     max_durability: i32,
//     defence: i32,
//     health_bonus: i32,
//     mana_bonus: i32,
// }
//
// struct PlayerStats {}
//
// trait Skill {
//     fn skill_name(&self) -> &str; // returns the name of the skill
//     fn skill_level(&self) -> &str; // returns the level of that skill
//     fn description(&self) -> &str; // returns the status of that skill
//     fn skillpoint_level_req(&self) -> i32;
//     fn increase_level(&mut self, level: &u32, skill_points: &mut u32) -> Result<(), GameError>;
//     fn decrease_level(&mut self, skill_points: &mut u32) -> Result<(), GameError>;
//     fn modify_stats(&self, stats: EntityStats) -> EntityStats;
// }
//
// enum GameError {
//     SkillLevelMaxed,
//     InsufficientSkillPoints,
//     InsufficientLevel,
// }
