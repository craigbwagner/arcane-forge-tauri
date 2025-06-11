use crate::errors::AppError;
use crate::repositories::character_repository;
use crate::services::mappings::character_service::CharacterService;
use rusqlite::Connection;

pub fn create(conn: &Connection) -> Result<i64, AppError> {
    let new_character = CharacterService::new()?;
    let id = character_repository::insert_character(conn, new_character)?;
    Ok(id)
}
