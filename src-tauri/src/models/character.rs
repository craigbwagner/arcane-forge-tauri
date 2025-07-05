use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub name: Option<String>,
    pub levels: Option<String>,
    pub creator: Option<String>,
    pub basic_description: Option<String>,
    pub combat_stats: Option<String>,
    pub languages: Option<String>,
    pub ability_scores: Option<String>,
    pub skills: Option<String>,
    pub kill_list: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
