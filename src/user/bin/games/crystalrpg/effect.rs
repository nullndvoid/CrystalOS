pub struct Effect {
    pub EffectType: EffectType,
    pub potency: i32,
    pub duration: Option<i32>,
}

pub enum EffectType {
    Poison,
    Regeneration,

    Harming,
    Healing,

    Speed,
    Slowness,
    Stunned,
    Confused,

    Strength,
    Weakness,

    OnFire,

    Invisible,
}