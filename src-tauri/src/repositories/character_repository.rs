use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};

use crate::{errors::AppError, models::character::Character, traits::repository::Repository};

pub struct CharacterRepository {
    db: Arc<Mutex<Connection>>,
}

impl Repository<Character> for CharacterRepository {
    fn get_all(&self) -> Result<Vec<Character>, AppError> {
        self.with_connection(|conn| {
            let mut stmt = conn
                .prepare("SELECT * FROM characters")
                .map_err(|e| AppError::DatabaseOperationError(e.to_string()))?;
            let character_iter = stmt
                .query_map([], |row| {
                    Ok(Character {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        creator: row.get(2)?,
                        basic_description: row.get(3)?,
                        languages: row.get(4)?,
                        ability_scores: row.get(5)?,
                        combat_stats: row.get(6)?,
                        skills: row.get(7)?,
                        kill_list: row.get(8)?,
                        created_at: row.get(9)?,
                        updated_at: row.get(10)?,
                    })
                })
                .map_err(|e| AppError::DatabaseOperationError(e.to_string()))?;

            let mut characters = Vec::new();
            for character in character_iter {
                characters.push(character?);
            }
            Ok(characters)
        })
    }

    fn get_by_id(&self, id: i64) -> Result<Option<Character>, AppError> {
        todo!()
    }

    fn insert(&self, character: Character) -> Result<i64, AppError> {
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

    fn update(&self, entity: Character) -> Result<(), AppError> {
        todo!()
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        todo!()
    }
}

impl CharacterRepository {
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

    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }
}
