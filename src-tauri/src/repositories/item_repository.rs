use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::{errors::AppError, models::item::Item, traits::repository::Repository};

pub struct ItemRepository {
    db: Arc<Mutex<SqliteConnection>>,
}

impl Repository<Item, NewItem> for ItemRepository {
    fn get_all(&self) -> std::result::Result<Vec<Item>, AppError> {
        todo!()
    }

    fn get_by_id(&self, id: i32) -> std::result::Result<Option<Item>, AppError> {
        todo!()
    }

    fn insert(&self, entity: NewItem) -> std::result::Result<i32, AppError> {
        todo!()
    }

    fn update(&self, entity: Item) -> std::result::Result<(), AppError> {
        todo!()
    }

    fn delete(&self, id: i32) -> std::result::Result<(), AppError> {
        todo!()
    }
}

impl ItemRepository {
    pub fn new(db: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { db }
    }
}
