use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::{
    errors::AppError,
    models::item::{Item, NewItem},
};

use super::Repository;

pub struct ItemRepository {
    db: Arc<Mutex<SqliteConnection>>,
}

impl Repository<Item, NewItem> for ItemRepository {
    fn get_all(conn: &Arc<Mutex<SqliteConnection>>) -> std::result::Result<Vec<Item>, AppError> {
        todo!()
    }

    fn get_by_id(
        conn: &Arc<Mutex<SqliteConnection>>,
        id: i32,
    ) -> std::result::Result<Item, AppError> {
        todo!()
    }

    fn insert(
        conn: &Arc<Mutex<SqliteConnection>>,
        entity: NewItem,
    ) -> std::result::Result<Item, AppError> {
        todo!()
    }

    fn update(
        conn: &Arc<Mutex<SqliteConnection>>,
        entity: Item,
    ) -> std::result::Result<(), AppError> {
        todo!()
    }

    fn delete(conn: &Arc<Mutex<SqliteConnection>>, id: i32) -> std::result::Result<(), AppError> {
        todo!()
    }
}
