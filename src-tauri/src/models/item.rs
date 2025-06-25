use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: Option<String>,
    pub description: Option<String>,
    pub total_charges: Option<u8>,
    pub value: Option<u16>,
    pub weight: Option<f32>,
    pub rarity: Option<String>,
    pub item_type: Option<String>,
    pub attunement: Option<bool>,
    pub is_magical: bool,
    pub properties: Option<String>,
    pub acquired_through: Option<String>,
}
