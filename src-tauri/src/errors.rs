use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("Failed to create database connection: {0}")]
    DatabaseConnectionError(String),
    #[error("Character creation failed: {0}")]
    CharacterCreationError(String),
    #[error("Failed to save character: {0}")]
    CharacterSaveError(String),
}
