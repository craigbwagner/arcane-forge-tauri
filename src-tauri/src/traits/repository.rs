use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

use crate::errors::AppError;

pub trait Repository<T, U> {
    fn get_all(conn: &Arc<Mutex<SqliteConnection>>) -> Result<Vec<T>, AppError>;
    fn get_by_id(conn: &Arc<Mutex<SqliteConnection>>, id: i32) -> Result<Option<T>, AppError>;
    fn insert(conn: &Arc<Mutex<SqliteConnection>>, entity: U) -> Result<T, AppError>;
    fn update(conn: &Arc<Mutex<SqliteConnection>>, entity: T) -> Result<(), AppError>;
    fn delete(conn: &Arc<Mutex<SqliteConnection>>, id: i32) -> Result<(), AppError>;
    fn get_connection(
        conn: &Arc<Mutex<SqliteConnection>>,
    ) -> Result<std::sync::MutexGuard<'_, SqliteConnection>, AppError> {
        conn.lock()
            .map_err(|e| AppError::DatabaseOperationError(format!("Lock error: {}", e)))
    }
}
