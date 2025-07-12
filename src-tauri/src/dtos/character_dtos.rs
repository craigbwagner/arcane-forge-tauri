use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub enum Sex {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub struct BasicDescription {
    pub race: String,
    pub sex: Sex,
    pub size: Size,
    pub age: u16,
    pub height: String,
    pub weight: u16,
    pub alignment: Alignment,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub struct CombatStats {
    pub initiative_mods: u8,
    pub speed: u8,
    pub speed_mods: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u8,
    pub hit_dice_remaining: u8,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub struct Skill {
    pub name: String,
    pub is_proficient: bool,
    pub has_expertise: bool,
    pub ability_name: Ability,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/app/types/character/")]
pub struct FullCharacterData {
    pub id: i32,
    pub name: String,
    pub creator: String,
    pub basic_description: BasicDescription,
    pub combat_stats: CombatStats,
    pub languages: Vec<String>,
    pub ability_scores: [AbilityScore; 6],
    pub skills: [Skill; 18],
    pub kill_list: Vec<String>,
    #[ts(type = "string")]
    pub created_at: DateTime<Utc>,
    #[ts(type = "string")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HitDiceType {
    D6,
    D8,
    D10,
    D12,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterClassDetails {
    pub class_name: String,
    pub level: u8,
    pub subclass: Option<String>,
    pub hit_die: HitDiceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterInventory {
    pub character_id: i32,
    pub items: Vec<CharacterItemDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterItemDetails {
    pub item_name: String,
    pub description: String,
    pub item_type: String,
    pub quantity: i32,
    pub equipped: bool,
    pub attuned: bool,
    pub weight: f32,
    pub value: i32,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterFeatureDetails {
    pub feature_name: String,
    pub description: String,
    pub feature_type: String,
    pub source: String,
    pub uses_remaining: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterSpellDetails {
    pub spell_name: String,
    pub level: i32,
    pub school: String,
    pub source_class: String,
    pub known: bool,
    pub always_prepared: bool,
    pub casting_time: String,
    pub range: String,
    pub duration: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        FullCharacterData::export().unwrap();
    }
}
