use rusqlite::Connection;

use crate::{errors::AppError, models::character::Character};

pub fn insert_character(conn: &Connection, dto: Character) -> Result<i64, AppError> {
    Ok(0)
}
