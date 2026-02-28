use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::errors::AppError;
use crate::models::character::Character;

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/character/")]
pub struct FullCharacterData {
    pub id: i32,
    pub name: String,
    pub creator: String,
    #[serde(skip_deserializing)]
    pub proficiency_bonus: u8,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "../../src/types/character/")]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../../src/types/character/")]
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
#[ts(export, export_to = "../../src/types/character/")]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, TS)]
#[ts(export, export_to = "../../src/types/character/")]
pub enum Sex {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/character/")]
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
            race: "Human".to_string(),
            sex: Sex::Unspecified,
            size: Size::Medium,
            age: 25,
            height: "5'8\"".to_string(),
            weight: 160,
            alignment: Alignment::TrueNeutral,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/character/")]
pub struct CombatStats {
    #[serde(skip_deserializing)]
    pub initiative: u8,
    pub initiative_mods: u8,
    pub speed: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u8,
    pub hit_dice_remaining: u8,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            initiative: 0,
            initiative_mods: 0,
            speed: 30,
            max_hp: 8,
            current_hp: 8,
            temp_hp: 0,
            hit_dice_remaining: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/character/")]
pub struct AbilityScore {
    pub name: String,
    pub short_name: String,
    pub is_proficient: bool,
    pub score: u8,
    pub additional_mods: u8,
    #[serde(skip_deserializing)]
    pub base_modifier: u8,
    #[serde(skip_deserializing)]
    pub total_mod: u8,
    #[serde(skip_deserializing)]
    pub save: u8,
    #[serde(skip_deserializing)]
    pub additional_save_mods: u8,
}

impl AbilityScore {
    pub fn calculate_modifier(score: u8) -> u8 {
        ((score as i8 - 10) / 2).max(0) as u8
    }

    pub fn update_calculated_fields(&mut self, proficiency_bonus: u8) {
        self.total_mod = Self::calculate_modifier(self.score) + self.additional_mods;
        self.save = if self.is_proficient {
            self.total_mod + self.additional_save_mods + proficiency_bonus
        } else {
            self.total_mod + self.additional_save_mods
        }
    }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/character/")]
pub struct Skill {
    pub name: String,
    pub is_proficient: bool,
    pub has_expertise: bool,
    pub ability_name: Ability,
    pub additional_mods: u8,
    #[serde(skip_deserializing)]
    pub total_mod: u8,
}

impl Skill {
    pub fn update_total_modifier(&mut self, abilities: &[AbilityScore; 6], proficiency_bonus: u8) {
        let ability_modifier = self.get_ability_modifier(abilities);

        self.total_mod = ability_modifier
            + self.additional_mods
            + if self.is_proficient {
                proficiency_bonus * if self.has_expertise { 2 } else { 1 }
            } else {
                0
            };
    }

    fn get_ability_modifier(&self, abilities: &[AbilityScore; 6]) -> u8 {
        match self.ability_name {
            Ability::Strength => abilities[0].total_mod,
            Ability::Dexterity => abilities[1].total_mod,
            Ability::Constitution => abilities[2].total_mod,
            Ability::Intelligence => abilities[3].total_mod,
            Ability::Wisdom => abilities[4].total_mod,
            Ability::Charisma => abilities[5].total_mod,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HitDiceType {
    D6,
    D8,
    D10,
    D12,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassDetails {
    pub class_name: String,
    pub level: u8,
    pub subclass: Option<String>,
    pub hit_die: HitDiceType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInventory {
    pub character_id: i32,
    pub items: Vec<CharacterItemDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CharacterFeatureDetails {
    pub feature_name: String,
    pub description: String,
    pub feature_type: String,
    pub source: String,
    pub uses_remaining: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

impl TryFrom<&Character> for FullCharacterData {
    type Error = AppError;

    fn try_from(data: &Character) -> Result<Self, AppError> {
        let basic_description: BasicDescription = serde_json::from_str(&data.basic_description)?;
        let combat_stats: CombatStats = serde_json::from_str(&data.combat_stats)?;
        let languages: Vec<String> = serde_json::from_str(&data.languages)?;
        let mut ability_scores: [AbilityScore; 6] = serde_json::from_str(&data.ability_scores)?;
        let mut skills: [Skill; 18] = serde_json::from_str(&data.skills)?;
        let kill_list: Vec<String> = serde_json::from_str(&data.kill_list)?;
        let proficiency_bonus = 2;

        ability_scores.iter_mut().for_each(|ability| {
            ability.update_calculated_fields(proficiency_bonus);
        });

        skills.iter_mut().for_each(|skill| {
            skill.update_total_modifier(&ability_scores, proficiency_bonus);
        });

        let created_at = DateTime::parse_from_rfc3339(&data.created_at)?.with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&data.updated_at)?.with_timezone(&Utc);

        Ok(FullCharacterData {
            id: data.id,
            name: data.name.clone(),
            creator: data.creator.clone(),
            proficiency_bonus,
            basic_description,
            combat_stats,
            languages,
            ability_scores,
            skills,
            kill_list,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn export_bindings() {
        FullCharacterData::export().unwrap();
    }
}
