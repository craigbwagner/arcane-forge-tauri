use tauri::State;

use crate::app_state::AppState;
use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::character_service;

#[tauri::command]
pub async fn get_all_characters(
    state: State<'_, AppState>,
) -> Result<Vec<FullCharacterData>, AppError> {
    character_service::get_all(state)
}

#[tauri::command]
pub async fn create_character(state: State<'_, AppState>) -> Result<FullCharacterData, AppError> {
    character_service::create(state)
}

#[tauri::command]
async fn update_character(
    data: FullCharacterData,
    state: State<'_, AppState>,
) -> Result<FullCharacterData, AppError> {
    character_service::update(data, state)
}
