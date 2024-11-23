use alloc::{boxed::Box, string::String, vec::Vec};

use super::{effect::Effect, items::{armour::{Boots, Chestplate, Helmet}, Item}};

pub struct Player {
    pub name: String,

    pub health: f64,
    pub max_health: f64,

    pub mana: f64,
    pub max_mana: f64,

    pub defence: f64,
    pub agility: f64,

    pub stamina: f64,
    pub max_stamina: f64,

    pub level: f64,
    pub experience: f64,
    pub skill_points: f64,

    pub helmet: Option<Helmet>,
    pub chestplate: Option<Chestplate>,
    pub boots: Option<Boots>,

    pub inventory: [Box<dyn Item>; 20],

    pub effects: Vec<Effect>,
}

impl Player {
    const BASE_MAX_HEALTH: f64 = 100.0;
    const BASE_MAX_MANA: f64 = 100.0;
    const BASE_MAX_STAMINA: f64 = 100.0;

    const BASE_DAMAGE: f64 = 0.0;
    const BASE_DEFENCE: f64 = 0.0;
    const BASE_AGILITY: f64 = 0.0;

    const BASE_LEVEL : f64 = 1.0;
}