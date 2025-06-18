use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::repositories::character_repository::CharacterRepository;
use crate::services::mappings::character_mapper;

pub fn get_all(repo: &CharacterRepository) -> Result<Vec<FullCharacterData>, AppError> {}

pub fn create() -> Result<FullCharacterData, AppError> {
    let new_character = character_mapper::new()?;
    Ok(new_character)
}

pub fn save_new(data: FullCharacterData, repo: &CharacterRepository) -> Result<i64, AppError> {
    let character_db_model = character_mapper::dto_to_db(data)?;
    repo.insert_character(character_db_model)
}
