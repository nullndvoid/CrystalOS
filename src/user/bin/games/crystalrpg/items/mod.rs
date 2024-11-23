
pub trait Item {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn sell_price(&self) -> i32;
    fn buy_price(&self) -> i32;
}

pub mod armour;
pub mod weapons;
pub mod potions;
