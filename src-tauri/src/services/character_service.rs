use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::mappings::character_mapper;
use rusqlite::Connection;

pub fn create() -> Result<FullCharacterData, AppError> {
    let new_character = character_mapper::new()?;
    Ok(new_character)
}

pub fn save_new(data: FullCharacterData, conn: &Connection) -> Result<i64, AppError> {
    let character_db = character_mapper::dto_to_db(&data);
    Ok(0)
}
