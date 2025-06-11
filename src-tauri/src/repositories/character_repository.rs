use rusqlite::{params, Connection, Result};

use crate::{errors::AppError, models::character::Character};

pub fn insert_character(conn: &Connection, character: Character) -> Result<i64, AppError> {
    dbg!(&character);
    let rows_affected = conn.execute(
        "INSERT INTO characters (
            name,
            creator,
            basic_description,
            languages,
            ability_scores,
            combat_stats,
            skills,
            kill_list,
            created_at,
            updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
        )",
        params![
            character.name,
            character.creator,
            character.basic_description,
            character.languages,
            character.ability_scores,
            character.combat_stats,
            character.skills,
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
