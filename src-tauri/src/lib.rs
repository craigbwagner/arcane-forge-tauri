use crate::{
    app_state::AppState, commands::character_commands,
    repositories::character_repository::CharacterRepository,
};

mod app_state;
mod commands;
mod db;
mod dtos;
mod errors;
mod models;
mod repositories;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let database = db::initialize()?;

    let app_state = AppState {
        character_repo: CharacterRepository::new(database.clone()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character_commands::create_character
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
