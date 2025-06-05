use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Character creation failed: {0}")]
    CharacterCreationError(String),
}
