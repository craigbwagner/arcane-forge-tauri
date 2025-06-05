use crate::models::character;

#[tauri::command]
pub fn create_character() -> Result<character::Character, String> {
    let new_character = character::Character::default();
    Ok(new_character)
}
