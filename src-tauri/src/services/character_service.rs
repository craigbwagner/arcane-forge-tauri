use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::mappings::character_mapper;
use rusqlite::Connection;

pub fn create(conn: &Connection) -> Result<FullCharacterData, AppError> {
    let new_character = character_mapper::new()?;
    Ok(new_character)
}
