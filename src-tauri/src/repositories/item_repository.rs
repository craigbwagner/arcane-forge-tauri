use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};

use crate::{errors::AppError, models::item::Item, traits::repository::Repository};

pub struct ItemRepository {
    db: Arc<Mutex<Connection>>,
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
