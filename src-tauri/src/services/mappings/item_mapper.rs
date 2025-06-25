use crate::dtos::item_dtos::{FullItemData, MagicItemRarity};

pub fn new() -> FullItemData {
    FullItemData {
        id: None,
        name: String::new(),
        description: String::new(),
        total_charges: 0,
        value: 0,
        weight: 0.0,
        rarity: MagicItemRarity::Common,
        item_type: String::new(),
        attunement: false,
        is_magical: false,
        acquired_through: String::new(),
    }
}
