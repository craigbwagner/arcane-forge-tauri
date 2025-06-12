use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::commands::character_commands;

mod commands;
mod dtos;
mod errors;
mod models;
mod repositories;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let db = Connection::open("arcane-forge.db")?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            levels TEXT,
            creator TEXT,
            basic_description TEXT,
            classes TEXT,
            languages TEXT,
            ability_scores TEXT,
            combat_stats TEXT,
            additional_features TEXT,
            skills TEXT,
            items TEXT,
            kill_list TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
        [],
    )?;

    let db = Arc::new(Mutex::new(db));

    tauri::Builder::default()
        .manage(db)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character_commands::create_character
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
