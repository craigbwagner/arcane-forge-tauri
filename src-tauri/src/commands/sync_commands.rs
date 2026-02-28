use tauri::State;

use crate::app_state::AppState;
use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::sync_service;

#[tauri::command]
pub async fn push_to_cloud(state: State<'_, AppState>) -> Result<usize, AppError> {
    let mongo_db = state
        .mongo
        .as_ref()
        .ok_or_else(|| AppError::SyncNotConfiguredError("MongoDB is not configured".to_string()))?;

    sync_service::push_to_cloud(&state.db, mongo_db).await
}

#[tauri::command]
pub async fn pull_from_cloud(
    state: State<'_, AppState>,
) -> Result<Vec<FullCharacterData>, AppError> {
    let mongo_db = state
        .mongo
        .as_ref()
        .ok_or_else(|| AppError::SyncNotConfiguredError("MongoDB is not configured".to_string()))?;

    sync_service::pull_from_cloud(&state.db, mongo_db).await
}

#[tauri::command]
pub async fn check_sync_status(state: State<'_, AppState>) -> Result<bool, AppError> {
    Ok(state.mongo.is_some())
}
