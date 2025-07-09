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
    #[error("Entity not found: {0}")]
    EntityNotFoundError(String),
    #[error("Entity creation failed: {0}")]
    EntityCreationError(String),
    #[error("Failed to save entity: {0}")]
    EntitySaveError(String),
    #[error("Date parsing error: {0}")]
    DateParseError(String),
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerializationError(e.to_string())
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(e: chrono::ParseError) -> Self {
        AppError::DateParseError(e.to_string())
    }
}
