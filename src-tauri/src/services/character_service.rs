use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::repositories::character_repository::CharacterRepository;
use crate::services::mappings::character_mapper;
use crate::traits::repository::Repository;

// pub fn get_all(repo: &CharacterRepository) -> Result<Vec<FullCharacterData>, AppError> {
//     let characters = repo.get_all();
// }

pub fn create() -> Result<FullCharacterData, AppError> {
    character_mapper::new()
}

pub fn save_new(data: FullCharacterData, repo: &CharacterRepository) -> Result<i32, AppError> {
    let character_db_model = character_mapper::dto_to_db(data)?;
    repo.insert(character_db_model)
}
