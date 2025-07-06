use crate::{app_state::AppState, commands::character_commands};

mod app_state;
mod commands;
mod db;
mod dtos;
mod errors;
mod models;
mod repositories;
mod schema;
mod services;
mod traits;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let database = db::initialize()?;

    let state = AppState { db: database };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character_commands::create_character,
            character_commands::get_all_characters
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
