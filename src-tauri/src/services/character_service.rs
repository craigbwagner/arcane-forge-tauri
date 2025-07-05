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

pub fn create() -> Result<FullCharacterData, AppError> {
    let new_character = character_mapper::new()?;
    character_mapper::db_to_dto(new_character)
}
