use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MagicItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: Option<String>,
    pub description: Option<String>,
    pub total_charges: Option<u8>,
    pub value: Option<u16>,
    pub weight: Option<f32>,
    pub rarity: Option<MagicItemRarity>,
    pub item_type: Option<String>, //weapon, armor, consumable, etc.
    pub attunement: Option<bool>,
    pub is_magical: Option<bool>,
    pub properties: Option<String>, // JSON for item-specific properties
    pub acquired_through: Option<String>,
}
