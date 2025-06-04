use crate::models::character;

#[tauri::command]
pub fn create() -> Result<(), String> {
    let ability_scores = arrange_ability_scores();
    Ok(())
}

fn arrange_ability_scores() -> [character::AbilityScore; 6] {
    const NAMES: [(&str, &str); 6] = [
        ("Strength", "STR"),
        ("Dexterity", "DEX"),
        ("Constitution", "CON"),
        ("Intelligence", "INT"),
        ("Wisdom", "WIS"),
        ("Charisma", "CHA"),
    ];

    NAMES.map(|(name, abbr)| character::AbilityScore::new(name.into(), abbr.into()))
}
