use chrono::{DateTime, Utc};

use crate::models::{character, class::Class, feature::Feature, item::Item};

pub struct CharacterDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub levels: Vec<character::LevelEntry>,
    pub creator: Option<String>,
    pub basic_description: character::BasicDescription,
    pub classes: Vec<Class>,
    pub languages: Vec<String>,
    pub ability_scores: Vec<character::AbilityScore>,
    pub combat_stats: character::CombatStats,
    pub additional_features: Vec<Feature>,
    pub skills: Vec<character::Skill>,
    pub items: Vec<Item>,
    pub kill_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
