use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: Option<i64>,
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
