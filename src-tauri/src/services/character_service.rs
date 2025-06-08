use crate::errors::AppError;
use crate::repositories::character_repository;
use crate::services::mappings::new_character;
use rusqlite::Connection;

pub fn create(conn: &Connection) -> Result<i64, AppError> {
    let new_character = new_character::new()?;
    let id = character_repository::insert_character(conn, new_character)?;
    Ok(id)
}
