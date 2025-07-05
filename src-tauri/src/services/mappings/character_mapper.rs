use chrono::{DateTime, Utc};

use crate::dtos::character_dtos::{
    Ability, AbilityScore, Alignment, BasicDescription, CharacterClassDetails, CharacterDetails,
    CharacterFeatureDetails, CharacterItemDetails, CharacterSpellDetails, CombatStats,
    FullCharacterData, Sex, Size, Skill,
};
use crate::errors::AppError;
use crate::models::character::{Character, NewCharacter};

pub fn new() -> Result<NewCharacter, AppError> {
    let now = Utc::now();

    let ability_scores = initial_ability_scores();
    let skills = initial_skills();
    let basic_description = serde_json::to_string(&initial_basic_description())?;
    let combat_stats = initial_combat_stats();
    let languages = Vec::<String>::new();
    let kill_list = Vec::<String>::new();

    let new_character = NewCharacter {
        name: String::new(),
        creator: String::new(),
        basic_description,
        levels: todo!(),
        combat_stats: todo!(),
        languages: todo!(),
        ability_scores: todo!(),
        skills: todo!(),
        kill_list: todo!(),
        created_at: todo!(),
        updated_at: todo!(),
    };

    Ok(new_character)
}

// pub fn dto_to_db(data: FullCharacterData) -> Result<Character, AppError> {
//     let basic_decription_json = serde_json::to_string(&data.character.basic_description)?;
//     let combat_stats_json = serde_json::to_string(&data.character.combat_stats)?;
//     let languages_json = serde_json::to_string(&data.character.languages)?;
//     let ability_scores_json = serde_json::to_string(&data.character.ability_scores)?;
//     let skills_json = serde_json::to_string(&data.character.skills)?;
//     let kill_list_json = serde_json::to_string(&data.character.kill_list)?;
//     let now = Utc::now().to_rfc3339();

//     let db_character = Character {
//         id: None,
//         name: data.character.name.clone(),
//         creator: data.character.creator.clone(),
//         basic_description: basic_decription_json,
//         combat_stats: combat_stats_json,
//         languages: languages_json,
//         ability_scores: ability_scores_json,
//         skills: skills_json,
//         kill_list: kill_list_json,
//         created_at: data.character.created_at.to_rfc3339(),
//         updated_at: now,
//     };

//     Ok(db_character)
// }

// pub fn db_to_dto(data: Character) -> Result<FullCharacterData, AppError> {
//     let basic_description: BasicDescription = serde_json::from_str(&data.basic_description)?;
//     let combat_stats: CombatStats = serde_json::from_str(&data.combat_stats)?;
//     let languages: Vec<String> = serde_json::from_str(&data.languages)?;
//     let ability_scores: [AbilityScore; 6] = serde_json::from_str(&data.ability_scores)?;
//     let skills: [Skill; 18] = serde_json::from_str(&data.skills)?;
//     let kill_list: Vec<String> = serde_json::from_str(&data.kill_list)?;
//     let created_at = parse_db_time(&data.created_at)?;
//     let updated_at = parse_db_time(&data.updated_at)?;

//     let character_response = FullCharacterData {
//         character: CharacterDetails {
//             id: data.id,
//             name: data.name,
//             creator: data.creator,
//             basic_description,
//             combat_stats,
//             languages,
//             ability_scores,
//             skills,
//             kill_list,
//             created_at,
//             updated_at,
//         },
//     };

//     Ok(character_response)
// }

// fn parse_db_time(date: &str) -> Result<DateTime<Utc>, AppError> {
//     DateTime::parse_from_rfc3339(&date[..])
//         .map(|datetime| datetime.to_utc())
//         .map_err(|e| AppError::SerializationError(e.to_string()))
// }

fn initial_ability_scores() -> [AbilityScore; 6] {
    let strength = AbilityScore {
        name: "Strength".into(),
        short_name: "STR".into(),
        is_proficient: false,
        score: 10,
    };
    let dexterity = AbilityScore {
        name: "Dexterity".into(),
        short_name: "DEX".into(),
        is_proficient: false,
        score: 10,
    };
    let constitution = AbilityScore {
        name: "Constitution".into(),
        short_name: "CON".into(),
        is_proficient: false,
        score: 10,
    };
    let intelligence = AbilityScore {
        name: "Intelligence".into(),
        short_name: "INT".into(),
        is_proficient: false,
        score: 10,
    };
    let wisdom = AbilityScore {
        name: "Wisdom".into(),
        short_name: "WIS".into(),
        is_proficient: false,
        score: 10,
    };
    let charisma = AbilityScore {
        name: "Charism".into(),
        short_name: "CHA".into(),
        is_proficient: false,
        score: 10,
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
    };
    let animal_handling = Skill {
        name: "Animal Handling".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
    };
    let arcana = Skill {
        name: "Arcana".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
    };
    let athletics = Skill {
        name: "Athletics".into(),
        ability_name: Ability::Strength,
        is_proficient: false,
        has_expertise: false,
    };
    let deception = Skill {
        name: "Deception".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
    };
    let history = Skill {
        name: "History".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
    };
    let insight = Skill {
        name: "Insight".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
    };
    let intimidation = Skill {
        name: "Intimidation".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
    };
    let investigation = Skill {
        name: "Investigation".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
    };
    let medicine = Skill {
        name: "Medicine".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
    };
    let nature = Skill {
        name: "Nature".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
    };
    let perception = Skill {
        name: "Perception".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
    };
    let performance = Skill {
        name: "Performance".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
    };
    let persuasion = Skill {
        name: "Persuasion".into(),
        ability_name: Ability::Charisma,
        is_proficient: false,
        has_expertise: false,
    };
    let religion = Skill {
        name: "Religion".into(),
        ability_name: Ability::Intelligence,
        is_proficient: false,
        has_expertise: false,
    };
    let sleight_of_hand = Skill {
        name: "Sleight of Hand".into(),
        ability_name: Ability::Dexterity,
        is_proficient: false,
        has_expertise: false,
    };
    let stealth = Skill {
        name: "Stealth".into(),
        ability_name: Ability::Dexterity,
        is_proficient: false,
        has_expertise: false,
    };
    let survival = Skill {
        name: "Survival".into(),
        ability_name: Ability::Wisdom,
        is_proficient: false,
        has_expertise: false,
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

fn initial_basic_description() -> BasicDescription {
    BasicDescription {
        race: String::new(),
        sex: Sex::Unspecified,
        size: Size::Medium,
        age: 0,
        height: String::new(),
        weight: 0,
        alignment: Alignment::TrueNeutral,
    }
}

fn initial_combat_stats() -> CombatStats {
    CombatStats {
        initiative_mods: 0,
        speed: 30,
        speed_mods: 0,
        max_hp: 0,
        current_hp: 0,
        temp_hp: 0,
        hit_dice_remaining: 0,
    }
}
