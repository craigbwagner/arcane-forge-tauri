use crate::{
    app_state::AppState,
    commands::{character_commands, sync_commands},
};

mod app_state;
mod commands;
mod db;
mod dtos;
mod errors;
mod models;
mod repositories;
mod schema;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let database = db::initialize()?;
    let mongo = db::mongo::initialize()?;

    let state = AppState {
        db: database,
        mongo,
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character_commands::get_all_characters,
            character_commands::get_character_by_id,
            character_commands::create_character,
            character_commands::delete_character,
            sync_commands::push_to_cloud,
            sync_commands::pull_from_cloud,
            sync_commands::check_sync_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
