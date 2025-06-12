use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tauri::{App, State};

use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::character_service;

#[tauri::command]
pub fn create_character() -> Result<FullCharacterData, AppError> {
    let new_character = character_service::create()?;
    Ok(new_character)
}

#[tauri::command]
async fn save_new_character(
    data: FullCharacterData,
    db: State<'_, Arc<Mutex<Connection>>>,
) -> Result<i64, AppError> {
    let conn = db
        .lock()
        .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?;

    let new_character_id = character_service::save_new_character(data, &conn)?;

    Ok(new_character_id)
}
