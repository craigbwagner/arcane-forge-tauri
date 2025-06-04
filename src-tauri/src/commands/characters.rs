use crate::models::character;

#[tauri::command]
pub fn create() -> Result<(), String> {
    let ability_scores = arrange_ability_scores();
    Ok(())
}

fn arrange_ability_scores() -> [character::AbilityScore; 6] {
    let strength = character::AbilityScore::new(String::from("Strength"), String::from("STR"));
    let dexterity = character::AbilityScore::new(String::from("Dexterity"), String::from("DEX"));
    let constitution =
        character::AbilityScore::new(String::from("Constitution"), String::from("CON"));
    let intelligence =
        character::AbilityScore::new(String::from("Intelligence"), String::from("INT"));
    let wisdom = character::AbilityScore::new(String::from("Wisdom"), String::from("WIS"));
    let charisma = character::AbilityScore::new(String::from("Charisma"), String::from("CHA"));

    [
        strength,
        dexterity,
        constitution,
        intelligence,
        wisdom,
        charisma,
    ]
}
