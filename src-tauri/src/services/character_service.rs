use crate::errors::AppError;
use crate::repositories::character_repository;
use crate::services::mappings::character_mapper::CharacterMapper;
use rusqlite::Connection;

pub fn create(conn: &Connection) -> Result<i64, AppError> {
    let new_character = CharacterMapper::new()?;
    let id = character_repository::insert_character(conn, new_character)?;
    Ok(id)
}
