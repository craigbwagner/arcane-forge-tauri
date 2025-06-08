use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

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

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alignment::LawfulGood => "Lawful Good",
                Alignment::NeutralGood => "Neutral Good",
                Alignment::ChaoticGood => "Chaotic Good",
                Alignment::LawfulNeutral => "Lawful Neutral",
                Alignment::TrueNeutral => "Neutral",
                Alignment::ChaoticNeutral => "Chaotic Neutral",
                Alignment::LawfulEvil => "Lawful Evil",
                Alignment::NeutralEvil => "Neutral Evil",
                Alignment::ChaoticEvil => "Chaotic Evil",
            }
        )
    }
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
pub enum HitDiceType {
    D6,
    D8,
    D10,
    D12,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub levels: String,
    pub creator: Option<String>,
    pub basic_description: String,
    pub classes: Vec<i64>,
    pub languages: Vec<String>,
    pub ability_scores: String,
    pub combat_stats: String,
    pub additional_features: Vec<i64>,
    pub skills: String,
    pub items: Vec<i64>,
    pub kill_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Character {
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CombatStats {
    pub initiative_mods: u8,
    pub speed: u8,
    pub speed_mods: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u8,
    pub hit_dice_remaining: u8,
    pub hit_dice_type: HitDiceType,
    pub hit_dice_total: u8,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            initiative_mods: 0,
            speed: 30,
            speed_mods: 0,
            max_hp: 1,
            current_hp: 1,
            temp_hp: 0,
            hit_dice_remaining: 1,
            hit_dice_type: HitDiceType::D8,
            hit_dice_total: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
}

impl AbilityScore {
    pub fn new(name: Ability, short_name: &str) -> Self {
        Self {
            name: format!("{:?}", name),
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
    pub ability_name: Ability,
}

impl Skill {
    pub fn new(name: &str, ability: Ability) -> Self {
        Self {
            name: String::from(name),
            is_proficient: false,
            has_expertise: false,
            ability_name: ability,
        }
    }
}
