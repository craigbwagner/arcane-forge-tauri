use serde::{Deserialize, Serialize};

pub mod character;
pub mod class;
pub mod feature;
pub mod item;
pub mod joins;
pub mod spell;

#[derive(Serialize, Deserialize, Debug)]
pub enum UseReset {
    Daily,
    LongRest,
    ShortRest,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    FreeAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}
