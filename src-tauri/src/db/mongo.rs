use dotenvy::var;
use mongodb::{Client, Database};

use crate::errors::AppError;

/// Initializes a MongoDB connection if `MONGODB_URI` is set in the environment.
///
/// Returns `Ok(None)` if the env var isn't present — the app runs fully offline.
/// Note: `Client::with_uri_str` only parses the URI; the actual TCP connection
/// happens lazily on the first operation (push/pull), so startup isn't blocked
/// even if Atlas is unreachable.
pub fn initialize() -> Result<Option<Database>, AppError> {
    let uri = match var("MONGODB_URI") {
        Ok(uri) => uri,
        Err(_) => return Ok(None),
    };

    let db_name = var("MONGODB_DB_NAME").unwrap_or_else(|_| "arcane_forge".to_string());

    let client = tauri::async_runtime::block_on(Client::with_uri_str(&uri))
        .map_err(|e| AppError::MongoConnectionError(e.to_string()))?;

    let database = client.database(&db_name);

    Ok(Some(database))
}
