use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::errors::AppError;

pub trait Repository<T> {
    fn get_all(&self, conn: &Arc<Mutex<SqliteConnection>>) -> Result<Vec<T>, AppError>;
    fn get_by_id(
        &self,
        conn: &Arc<Mutex<SqliteConnection>>,
        id: i32,
    ) -> Result<Option<T>, AppError>;
    fn insert(&self, conn: &Arc<Mutex<SqliteConnection>>, entity: T) -> Result<i32, AppError>;
    fn update(&self, conn: &Arc<Mutex<SqliteConnection>>, entity: T) -> Result<(), AppError>;
    fn delete(&self, conn: &Arc<Mutex<SqliteConnection>>, id: i32) -> Result<(), AppError>;
}
