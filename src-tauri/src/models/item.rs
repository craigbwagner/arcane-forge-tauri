use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub name: String,
    pub description: String,
    pub total_charges: i32,
    pub value: i32,
    pub weight: f32,
    pub rarity: String,
    pub item_type: String,
    pub attunement: bool,
    pub is_magical: bool,
    pub acquired_through: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[diesel(table_name = crate::schema::items)]
pub struct NewItem {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub total_charges: i32,
    pub value: i32,
    pub weight: f32,
    pub rarity: String,
    pub item_type: String,
    pub attunement: bool,
    pub is_magical: bool,
    pub acquired_through: String,
}
