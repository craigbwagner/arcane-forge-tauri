use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::{errors::AppError, models::item::Item, traits::repository::Repository};

pub struct ItemRepository {
    db: Arc<Mutex<SqliteConnection>>,
}

impl Repository<Item> for ItemRepository {
    fn get_all(&self) -> std::result::Result<Vec<Item>, AppError> {
        todo!()
    }

    fn get_by_id(&self, id: i64) -> std::result::Result<Option<Item>, AppError> {
        todo!()
    }

    fn insert(&self, entity: Item) -> std::result::Result<i64, AppError> {
        todo!()
    }

    fn update(&self, entity: Item) -> std::result::Result<(), AppError> {
        todo!()
    }

    fn delete(&self, id: i64) -> std::result::Result<(), AppError> {
        todo!()
    }
}

impl ItemRepository {
    fn with_connection<T, F>(&self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(&SqliteConnection) -> Result<T, AppError>,
    {
        let conn = self
            .db
            .lock()
            .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;
        f(&*conn)
    }

    pub fn new(db: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { db }
    }
}
