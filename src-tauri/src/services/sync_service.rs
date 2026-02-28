use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;
use mongodb::Database;

use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::models::character::NewCharacter;
use crate::repositories::character_repository::CharacterRepository;
use crate::repositories::cloud_character_repository::CloudCharacterRepository;
use crate::traits::repository::Repository;

/// Reads all characters from SQLite, maps them to DTOs, and replaces
/// the entire MongoDB collection with the result.
/// Returns the number of characters pushed.
pub async fn push_to_cloud(
    db: &Arc<Mutex<SqliteConnection>>,
    mongo_db: &Database,
) -> Result<usize, AppError> {
    // SQLite read — lock is acquired and released within get_all()
    let db_characters = CharacterRepository::get_all(db)?;

    let dtos: Vec<FullCharacterData> = db_characters
        .iter()
        .map(FullCharacterData::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let count = dtos.len();
    CloudCharacterRepository::replace_all(mongo_db, dtos).await?;

    Ok(count)
}

/// Fetches all characters from MongoDB, converts them to NewCharacter
/// records, clears the local SQLite table, inserts all, and returns
/// fresh DTOs with recalculated fields.
pub async fn pull_from_cloud(
    db: &Arc<Mutex<SqliteConnection>>,
    mongo_db: &Database,
) -> Result<Vec<FullCharacterData>, AppError> {
    // MongoDB read (async)
    let cloud_characters = CloudCharacterRepository::get_all(mongo_db).await?;

    let new_characters: Vec<_> = cloud_characters
        .iter()
        .map(NewCharacter::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    // SQLite write — each call acquires/releases the lock independently
    CharacterRepository::delete_all(db)?;
    let inserted = CharacterRepository::insert_many(db, new_characters)?;

    // Map back to DTOs so calculated fields are recomputed
    inserted
        .iter()
        .map(FullCharacterData::try_from)
        .collect()
}
