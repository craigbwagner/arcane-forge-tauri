use tauri::State;

use crate::app_state::AppState;
use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::models::character::NewCharacter;
use crate::repositories::character_repository::CharacterRepository;
use crate::traits::repository::Repository;

pub fn get_all(state: State<'_, AppState>) -> Result<Vec<FullCharacterData>, AppError> {
    let characters = CharacterRepository::get_all(&state.db)?;
    characters
        .iter()
        .map(FullCharacterData::try_from)
        .collect()
}

pub fn get_by_id(state: State<'_, AppState>, id: i32) -> Result<FullCharacterData, AppError> {
    let character = CharacterRepository::get_by_id(&state.db, id)?;
    FullCharacterData::try_from(&character)
}

pub fn create(state: State<'_, AppState>) -> Result<FullCharacterData, AppError> {
    let new_character_data = NewCharacter::new_default()?;
    let new_character = CharacterRepository::insert(&state.db, new_character_data)?;
    FullCharacterData::try_from(&new_character)
}

pub fn update(
    data: FullCharacterData,
    state: State<'_, AppState>,
) -> Result<FullCharacterData, AppError> {
    todo!()
}

pub fn delete(state: State<'_, AppState>, id: i32) -> Result<bool, AppError> {
    match CharacterRepository::delete(&state.db, id) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
