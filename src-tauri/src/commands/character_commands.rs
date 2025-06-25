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
pub async fn create_character() -> Result<FullCharacterData, AppError> {
    character_service::create()
}

#[tauri::command]
async fn save_character(
    data: FullCharacterData,
    app_state: State<'_, AppState>,
) -> Result<i64, AppError> {
    character_service::save_new(data, &app_state.character_repo)
}
