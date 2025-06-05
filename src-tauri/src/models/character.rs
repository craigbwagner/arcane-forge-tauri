use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub creator: String,
    pub basic_description: BasicDescription,
    pub classes: Vec<Class>,
    pub languages: Vec<String>,
    pub ability_scores: [AbilityScore; 6],
    pub combat_stats: CombatStats,
    pub additional_features: Vec<AdditionalFeature>,
    pub skills: [Skill; 18],
    pub class_features: Vec<ClassFeature>,
    pub subclass_features: Vec<SubclassFeature>,
    pub items: String,
}

#[derive(Serialize, Deserialize)]
pub struct BasicDescription {
    pub race: String,
    pub sex: String,
    pub size: String,
    pub age: u16,
    pub height: String,
    pub weight: u16,
    pub alignment: String,
}

#[derive(Serialize, Deserialize)]
pub struct CombatStats {
    pub initiative_mods: u8,
    pub speed: u8,
    pub speed_mods: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u8,
    pub hit_dice_remaining: u8,
    pub hit_dice_type: String,
    pub hit_dice_total: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub subclass: String,
    pub level: u8,
    pub class_features: Vec<ClassFeature>,
}

#[derive(Serialize, Deserialize)]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
}

impl AbilityScore {
    pub fn new(name: &str, short_name: &str) -> Self {
        Self {
            name: String::from(name),
            short_name: String::from(short_name),
            is_proficient: false,
            score: 10,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    name: String,
    is_proficient: bool,
    has_expertise: bool,
    ability_name: String,
}

impl Skill {
    pub fn new(name: &str, ability_name: &str) -> Self {
        Self {
            name: String::from(name),
            is_proficient: false,
            has_expertise: false,
            ability_name: String::from(ability_name),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassFeature {}

#[derive(Serialize, Deserialize)]
pub struct SubclassFeature {}

#[derive(Serialize, Deserialize)]
pub struct AdditionalFeature {}

#[derive(Serialize, Deserialize)]
pub struct Item {}
