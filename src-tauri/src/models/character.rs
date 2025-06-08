use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicDescription {
    pub race: String,
    pub sex: Sex,
    pub size: Size,
    pub age: u16,
    pub height: String,
    pub weight: u16,
    pub alignment: Alignment,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub is_proficient: bool,
    pub has_expertise: bool,
    pub ability_name: Ability,
}
