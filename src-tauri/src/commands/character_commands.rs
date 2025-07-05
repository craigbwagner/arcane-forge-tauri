use tauri::State;

use crate::app_state::AppState;
use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;
use crate::services::character_service;

// #[tauri::command]
// pub async fn get_all_characters(
//     app_state: State<'_, AppState>,
// ) -> Result<Vec<FullCharacterData>, AppError> {
//     character_service::get_all(&app_state.character_repo)
// }

#[tauri::command]
pub async fn create_character(state: State<'_, AppState>) -> Result<FullCharacterData, AppError> {
    character_service::create(state)
}

#[tauri::command]
async fn save_character(
    data: FullCharacterData,
    state: State<'_, AppState>,
) -> Result<i32, AppError> {
    character_service::save(data, state)
}
