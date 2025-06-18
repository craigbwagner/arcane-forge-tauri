use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};

use crate::{errors::AppError, models::character::Character};

pub struct CharacterRepository {
    db: Arc<Mutex<Connection>>,
}

impl CharacterRepository {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }

    fn with_connection<T, F>(&self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(&Connection) -> Result<T, AppError>,
    {
        let conn = self
            .db
            .lock()
            .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;
        f(&*conn)
    }

    // pub fn get_all(conn: &Connection) -> Result<Vec<Character>, AppError {

    // }

    pub fn insert_character(&self, character: Character) -> Result<i64, AppError> {
        self.with_connection(|conn| {
            let query_result = conn.execute(
                "INSERT INTO characters (
                name,
                creator,
                basic_description,
                languages,
                ability_scores,
                combat_stats,
                skills,
                kill_list,
                created_at,
                updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
            )",
                params![
                    character.name,
                    character.creator,
                    character.basic_description,
                    character.languages,
                    character.ability_scores,
                    character.combat_stats,
                    character.skills,
                    character.kill_list,
                    character.created_at,
                    character.updated_at,
                ],
            );
            match query_result {
                Ok(_) => Ok(conn.last_insert_rowid()),
                Err(e) => Err(AppError::CharacterCreationError(format!(
                    "Failed to insert character in db: {}",
                    e
                ))),
            }
        })
    }
}
