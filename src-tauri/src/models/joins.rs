use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterItem {
    pub id: Option<i64>,
    pub character_id: i64,
    pub item_id: i64,
    pub quantity: u32,
    pub equipped: bool,
    pub attuned: bool,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterSpell {
    pub id: Option<i64>,
    pub character_id: i64,
    pub spell_id: i64,
    pub source_class_id: Option<i64>, // which class provides access to this spell
    pub known: bool,                  // true if known, false if prepared
    pub always_prepared: bool,        // for domain spells, etc.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterFeature {
    pub id: Option<i64>,
    pub character_id: i64,
    pub feature_id: i64,
    pub acquired_at_level: Option<u8>,
    pub alternate_source: Option<String>, // for custom features, etc.
    pub source_class_id: Option<i64>,     // if from a class
    pub uses_remaining: Option<u8>,       // for limited-use features
    pub max_uses: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterClass {
    pub id: Option<i64>,
    pub character_id: i64,
    pub class_id: i64,
    pub level: i32,
    pub subclass: Option<String>,
}
