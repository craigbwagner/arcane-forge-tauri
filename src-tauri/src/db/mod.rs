use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::{Arc, Mutex};

use crate::errors::AppError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct Database {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub fn new() -> Result<Self, AppError> {
        let database_url = "arcane-forge.db";
        let mut connection = SqliteConnection::establish(database_url)
            .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;

        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;

        Ok(Database {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn get_connection(&self) -> Arc<Mutex<SqliteConnection>> {
        Arc::clone(&self.connection)
    }
}
