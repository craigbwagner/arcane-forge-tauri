use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Sex {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub basic_description: BasicDescription,
    pub classes: Vec<Class>,
    pub languages: Vec<String>,
    pub ability_scores: [AbilityScore; 6],
    pub combat_stats: CombatStats,
    pub additional_features: Vec<Feature>,
    pub skills: [Skill; 18],
    pub items: Vec<Item>,
    pub kill_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicDescription {
    pub race: String,
    pub sex: Sex,
    pub size: Size,
    pub age: u16,
    pub height: String,
    pub weight: u16,
    pub alignment: Alignment,
}

impl Default for BasicDescription {
    fn default() -> Self {
        Self {
            race: String::new(),
            sex: Sex::Unspecified,
            size: Size::Medium,
            age: 0,
            height: String::new(),
            weight: 0,
            alignment: Alignment::TrueNeutral,
        }
    }
}

impl Character {
    pub fn total_level(&self) -> u8 {
        self.classes.iter().map(|c| c.level).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Class {
    pub name: String,
    pub subclass: String,
    pub level: u8,
    pub class_features: Vec<Feature>,
    pub subclass_features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Skill {
    pub name: String,
    pub is_proficient: bool,
    pub has_expertise: bool,
    pub ability_name: String,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Feature {
    pub name: Option<String>,
    pub description: Option<String>,
    pub uses: Option<u8>,
    pub uses_reset_on: Option<u8>,
    pub action_type: Option<String>,
    pub duration: Option<String>,
    pub source: Option<String>,
    pub level_acquired: Option<u8>,
    pub is_passive: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub name: Option<String>,
    pub description: Option<String>,
    pub charges: Option<u8>,
    pub value: Option<u16>,
    pub weight: Option<f32>,
    pub rarity: Option<String>,
    pub item_type: Option<String>,
    pub properties: Option<Vec<String>>,
    pub attunement: Option<bool>,
    pub is_magical: Option<bool>,
    pub source: Option<String>,
    pub acquired_through: Option<String>,
}
