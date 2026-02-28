pub mod mongo;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::{dotenv, var};
use std::sync::{Arc, Mutex};

use crate::errors::AppError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn initialize() -> Result<Arc<Mutex<SqliteConnection>>, AppError> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").unwrap_or_else(|_| "arcane-forge.db".to_string());
    let mut connection = SqliteConnection::establish(&database_url)
        .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| AppError::DatabaseOperationError(e.to_string()))?;

    Ok(Arc::new(Mutex::new(connection)))
}
