use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub total_charges: u8,
    pub value: u16,
    pub weight: f32,
    pub rarity: String,
    pub item_type: String,
    pub attunement: bool,
    pub is_magical: bool,
    pub acquired_through: String,
}
