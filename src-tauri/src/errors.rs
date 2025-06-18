use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("Failed to create database connection: {0}")]
    DatabaseConnectionError(String),
    #[error("Database operation failed: {0}")]
    DatabaseOperationError(String),
    #[error("Mapping operation failed: {0}")]
    MappingError(String),
    #[error("Data serialization failed: {0}")]
    SerializationError(String),
    #[error("Character creation failed: {0}")]
    CharacterCreationError(String),
    #[error("Failed to save character: {0}")]
    CharacterSaveError(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::DatabaseOperationError(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerializationError(e.to_string())
    }
}
