


use super::{map::Map, player::Player};

pub struct Game {
    pub score: u32,
    pub player: Player,
    pub map: Map,
}