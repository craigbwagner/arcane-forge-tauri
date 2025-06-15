use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::commands::character_commands;

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
