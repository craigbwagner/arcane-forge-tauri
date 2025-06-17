use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::errors::AppError;

pub fn initialize() -> Result<Arc<Mutex<Connection>>, AppError> {
    let conn = match Connection::open("arcane-forge.db") {
        Ok(conn) => conn,
        Err(e) => return Err(AppError::DatabaseConnectionError(e.to_string())),
    };

    conn.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            levels TEXT,
            creator TEXT,
            basic_description TEXT,
            classes TEXT,
            languages TEXT,
            ability_scores TEXT,
            combat_stats TEXT,
            additional_features TEXT,
            skills TEXT,
            items TEXT,
            kill_list TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
        [],
    )
    .map_err(|e| AppError::DatabaseOperationError(e.to_string()))?;

    Ok(Arc::new(Mutex::new(conn)))
}
