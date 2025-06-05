use crate::models::character;

#[tauri::command]
pub fn create() -> Result<character::Character, String> {
    let new_character = character::Character::default();
    Ok(new_character)
}
