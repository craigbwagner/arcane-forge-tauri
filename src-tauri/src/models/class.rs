use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub class_features: String,
    pub subclasses: String,
    pub hit_die: String,
    pub primary_ability: String, // dex, str, char, int, wis, or
    pub saving_throw_proficiencies: String, // deserialized into string
}
