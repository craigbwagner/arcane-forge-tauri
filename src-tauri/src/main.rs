// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use color_eyre::eyre;

fn main() -> Result<(), eyre::Report> {
    arcane_forge_lib::run()
}
