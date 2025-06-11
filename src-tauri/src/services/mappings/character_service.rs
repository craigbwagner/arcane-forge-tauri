use chrono::Utc;

use crate::dtos::character_dtos::CharacterDto;
use crate::errors::AppError;
use crate::models::character::{
    Ability, AbilityScore, Alignment, BasicDescription, Character, CombatStats, HitDiceType,
    LevelEntry, Sex, Size, Skill,
};

pub struct CharacterService;

impl CharacterService {
    pub fn new() -> Result<Character, AppError> {
        let now = Utc::now();

        let ability_scores = serde_json::to_string(&Self::initial_ability_scores())
            .expect("Should have deserialized initial ability scores.");
        let skills = serde_json::to_string(&Self::initial_skills())
            .expect("Should have deserialized initial skills.");
        let levels = serde_json::to_string(&Self::initial_levels())
            .expect("Should have deserialized initial levels.");
        let basic_description = serde_json::to_string(&Self::initial_basic_description())
            .expect("Should have deserialized initial basic description.");
        let combat_stats = serde_json::to_string(&Self::initial_combat_stats())
            .expect("Should have deserialized initial combat stats.");
        let additional_features = serde_json::to_string(&Vec::<i64>::new())
            .expect("Should have deserialized initial features.");
        let items = serde_json::to_string(&Vec::<i64>::new())
            .expect("Should have deserialized initial items.");
        let kill_list = serde_json::to_string(&Vec::<String>::new())
            .expect("Should have deserialized initial kill list.");

        let new_character = Character {
            id: None,
            name: String::new(),
            levels,
            creator: String::new(),
            basic_description,
            classes: String::new(),
            languages: String::new(),
            ability_scores,
            combat_stats,
            additional_features,
            skills,
            items,
            kill_list,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
        };

        Ok(new_character)
    }

    // pub fn dto_to_db(dto: &CharacterDto) -> Character {
    //     let basic_description_json = serde_json::to_string(&dto.basic_description);
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

    fn initial_levels() -> Vec<LevelEntry> {
        let levels = vec![LevelEntry {
            class: None,
            level: 1,
            subclass: None,
        }];
        levels
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
            hit_dice_type: HitDiceType::D8,
            hit_dice_total: 0,
        }
    }
}
