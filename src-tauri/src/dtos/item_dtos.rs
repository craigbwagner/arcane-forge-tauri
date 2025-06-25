use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MagicItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FullItemData {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub total_charges: u8,
    pub value: u16,
    pub weight: f32,
    pub rarity: MagicItemRarity,
    pub item_type: String,
    pub attunement: bool,
    pub is_magical: bool,
    pub acquired_through: String,
}
