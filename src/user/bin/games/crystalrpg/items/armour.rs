pub struct Helmet {
    name: &'static str,
    lore: &'static str,
    stats: ArmourStats,
}

pub struct Chestplate {
    name: &'static str,
    lore: &'static str,
    stats: ArmourStats,
}

pub struct Boots {
    name: &'static str,
    lore: &'static str,
    stats: ArmourStats,
}

pub struct ArmourStats {
    defence: i32,
    health_bonus: i32,
    mana_bonus: i32,
}
