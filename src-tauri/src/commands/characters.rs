use crate::models::character;

#[tauri::command]
pub fn create() -> Result<(), String> {
    let ability_scores = arrange_ability_scores();
    let skills = arrange_skills();
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

    NAMES.map(|(name, short_name)| character::AbilityScore::new(name, short_name))
}

fn arrange_skills() -> [character::Skill; 18] {
    const SKILLS: [(&str, &str); 18] = [
        ("Acrobatics", "DEX"),
        ("Animal Handling", "WIS"),
        ("Arcana", "INT"),
        ("Athletics", "STR"),
        ("Deception", "CHA"),
        ("History", "INT"),
        ("Insight", "WIS"),
        ("Intimidation", "CHA"),
        ("Investigation", "INT"),
        ("Medicine", "WIS"),
        ("Nature", "INT"),
        ("Perception", "WIS"),
        ("Performance", "CHA"),
        ("Persuasion", "CHA"),
        ("Religion", "INT"),
        ("Sleight of Hand", "DEX"),
        ("Stealth", "DEX"),
        ("Survival", "WIS"),
    ];

    SKILLS.map(|(name, ability)| character::Skill::new(name, ability))
}
