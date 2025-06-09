use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::commands::characters::create_character;

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
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            alignment TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    let db = Arc::new(Mutex::new(db));

    tauri::Builder::default()
        .manage(db)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_character])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
