use crate::models::character;

#[tauri::command]
pub fn create_character() -> Result<character::Character, String> {
    let new_character = character::Character::default();
    Ok(new_character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_result_is_ok() {
        let result = create_character();
        assert!(result.is_ok());
    }
}
