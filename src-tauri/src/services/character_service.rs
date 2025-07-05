use tauri::State;

use crate::app_state::AppState;
use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::repositories::character_repository::CharacterRepository;
use crate::services::mappings::character_mapper;
use crate::traits::repository::Repository;

// pub fn get_all(repo: &CharacterRepository) -> Result<Vec<FullCharacterData>, AppError> {
//     let characters = repo.get_all();
// }

pub fn create(state: State<'_, AppState>) -> Result<FullCharacterData, AppError> {
    let new_character_data = character_mapper::new()?;
    let new_character = CharacterRepository::insert(&state.db, new_character_data)?;
    character_mapper::db_to_dto(new_character)
}

pub fn update(
    data: FullCharacterData,
    state: State<'_, AppState>,
) -> Result<FullCharacterData, AppError> {
    todo!()
}
