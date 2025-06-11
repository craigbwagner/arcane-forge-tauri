use rusqlite::{params, Connection, Result};
use tauri::App;

use crate::{errors::AppError, models::character::Character};

pub fn insert_character(conn: &Connection, character: Character) -> Result<i64, AppError> {
    dbg!(&character);
    let rows_affected = conn.execute(
        "INSERT INTO characters (
            name,
            levels,
            creator,
            basic_description,
            classes,
            languages,
            ability_scores,
            combat_stats,
            additional_features,
            skills,
            items,
            kill_list,
            created_at,
            updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14
        )",
        params![
            character.name,
            character.levels,
            character.creator,
            character.basic_description,
            character.classes,
            character.languages,
            character.ability_scores,
            character.combat_stats,
            character.additional_features,
            character.skills,
            character.items,
            character.kill_list,
            character.created_at,
            character.updated_at,
        ],
    );

    match rows_affected {
        Err(e) => Err(AppError::CharacterCreationError(format!(
            "Failed to insert character in db: {}",
            e
        ))),
        Ok(_) => Ok(conn.last_insert_rowid()),
    }
}
