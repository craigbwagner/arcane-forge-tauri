use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::{errors::AppError, models::character::Character, traits::repository::Repository};

pub struct CharacterRepository {
    db: Arc<Mutex<SqliteConnection>>,
}

impl Repository<Character> for CharacterRepository {
    fn get_all(&self) -> Result<Vec<Character>, AppError> {
        todo!()
    }

    fn get_by_id(&self, id: i32) -> Result<Option<Character>, AppError> {
        todo!()
    }

    fn insert(&self, character: Character) -> Result<i32, AppError> {
        todo!()
    }

    fn update(&self, entity: Character) -> Result<(), AppError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), AppError> {
        todo!()
    }
}

impl CharacterRepository {
    // fn with_connection<T, F>(&self, f: F) -> Result<T, AppError>
    // where
    //     F: FnOnce(&SqliteConnection) -> Result<T, AppError>,
    // {
    //     let conn = self
    //         .db
    //         .lock()
    //         .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;
    //     f(&*conn)
    // }

    pub fn new(db: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { db }
    }
}
