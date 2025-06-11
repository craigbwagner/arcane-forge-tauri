use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::character_service;

#[tauri::command]
pub fn create_character(
    db: State<'_, Arc<Mutex<Connection>>>,
) -> Result<FullCharacterData, AppError> {
    let conn = db.lock().unwrap();
    let new_character = character_service::create(&conn)?;
    Ok(new_character)
}
