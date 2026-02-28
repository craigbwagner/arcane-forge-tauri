use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::dtos::character_dtos::{
    Ability, AbilityScore, BasicDescription, CombatStats, FullCharacterData, Skill,
};
use crate::errors::AppError;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub creator: String,
    pub basic_description: String,
    pub combat_stats: String,
    pub languages: String,
    pub ability_scores: String,
    pub skills: String,
    pub kill_list: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::characters)]
pub struct NewCharacter {
    pub name: String,
    pub creator: String,
    pub basic_description: String,
    pub combat_stats: String,
    pub languages: String,
    pub ability_scores: String,
    pub skills: String,
    pub kill_list: String,
    pub created_at: String,
    pub updated_at: String,
}

impl NewCharacter {
    pub fn new_default() -> Result<Self, AppError> {
        let now = Utc::now().to_rfc3339();

        Ok(NewCharacter {
            name: "New Character".to_string(),
            creator: String::new(),
            basic_description: serde_json::to_string(&BasicDescription::default())?,
            combat_stats: serde_json::to_string(&CombatStats::default())?,
            languages: serde_json::to_string(&Vec::<String>::new())?,
            ability_scores: serde_json::to_string(&initial_ability_scores())?,
            skills: serde_json::to_string(&initial_skills())?,
            kill_list: serde_json::to_string(&Vec::<String>::new())?,
            created_at: now.clone(),
            updated_at: now,
        })
    }
}

impl TryFrom<&FullCharacterData> for NewCharacter {
    type Error = AppError;

    fn try_from(data: &FullCharacterData) -> Result<Self, AppError> {
        Ok(NewCharacter {
            name: data.name.clone(),
            creator: data.creator.clone(),
            basic_description: serde_json::to_string(&data.basic_description)?,
            combat_stats: serde_json::to_string(&data.combat_stats)?,
            languages: serde_json::to_string(&data.languages)?,
            ability_scores: serde_json::to_string(&data.ability_scores)?,
            skills: serde_json::to_string(&data.skills)?,
            kill_list: serde_json::to_string(&data.kill_list)?,
            created_at: data.created_at.to_rfc3339(),
            updated_at: data.updated_at.to_rfc3339(),
        })
    }
}

fn initial_ability_scores() -> [AbilityScore; 6] {
    let strength = AbilityScore {
        name: "Strength".into(),
        short_name: "STR".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };
    let dexterity = AbilityScore {
        name: "Dexterity".into(),
        short_name: "DEX".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };
    let constitution = AbilityScore {
        name: "Constitution".into(),
        short_name: "CON".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };
    let intelligence = AbilityScore {
        name: "Intelligence".into(),
        short_name: "INT".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };
    let wisdom = AbilityScore {
        name: "Wisdom".into(),
        short_name: "WIS".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };
    let charisma = AbilityScore {
        name: "Charisma".into(),
        short_name: "CHA".into(),
        is_proficient: false,
        score: 10,
        base_modifier: 0,
        additional_mods: 0,
        total_mod: 0,
        additional_save_mods: 0,
        save: 0,
    };

    [
        strength,
        dexterity,
        constitution,
        intelligence,
        wisdom,
        charisma,
    ]
}

fn initial_skills() -> [Skill; 18] {
    let acrobatics = Skill {
        name: "Acrobatics".into(),
        ability_name: Ability::Dexterity,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let animal_handling = Skill {
        name: "Animal Handling".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let arcana = Skill {
        name: "Arcana".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let athletics = Skill {
        name: "Athletics".into(),
        ability_name: Ability::Strength,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let deception = Skill {
        name: "Deception".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let history = Skill {
        name: "History".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let insight = Skill {
        name: "Insight".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let intimidation = Skill {
        name: "Intimidation".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let investigation = Skill {
        name: "Investigation".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let medicine = Skill {
        name: "Medicine".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let nature = Skill {
        name: "Nature".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let perception = Skill {
        name: "Perception".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let performance = Skill {
        name: "Performance".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let persuasion = Skill {
        name: "Persuasion".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let religion = Skill {
        name: "Religion".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let sleight_of_hand = Skill {
        name: "Sleight of Hand".into(),
        ability_name: Ability::Dexterity,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let stealth = Skill {
        name: "Stealth".into(),
        ability_name: Ability::Dexterity,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };
    let survival = Skill {
        name: "Survival".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
        additional_mods: 0,
        total_mod: 0,
    };

    [
        acrobatics,
        animal_handling,
        arcana,
        athletics,
        deception,
        history,
        insight,
        intimidation,
        investigation,
        medicine,
        nature,
        perception,
        performance,
        persuasion,
        religion,
        sleight_of_hand,
        stealth,
        survival,
    ]
}
