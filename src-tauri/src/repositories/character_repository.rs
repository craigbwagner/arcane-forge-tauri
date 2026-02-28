use std::sync::{Arc, Mutex};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::{
    errors::AppError,
    models::character::{Character, NewCharacter},
    schema::characters,
};

use super::Repository;

pub struct CharacterRepository;

impl CharacterRepository {
    pub fn delete_all(conn: &Arc<Mutex<SqliteConnection>>) -> Result<(), AppError> {
        let mut conn = conn
            .lock()
            .map_err(|e| AppError::DatabaseOperationError(format!("Lock error: {}", e)))?;

        diesel::delete(characters::table)
            .execute(&mut *conn)
            .map_err(|e| AppError::DatabaseOperationError(e.to_string()))?;

        Ok(())
    }

    pub fn insert_many(
        conn: &Arc<Mutex<SqliteConnection>>,
        new_characters: Vec<NewCharacter>,
    ) -> Result<Vec<Character>, AppError> {
        let mut conn = conn
            .lock()
            .map_err(|e| AppError::DatabaseOperationError(format!("Lock error: {}", e)))?;

        let mut results = Vec::with_capacity(new_characters.len());
        for character in new_characters {
            let inserted = diesel::insert_into(characters::table)
                .values(character)
                .returning(Character::as_returning())
                .get_result(&mut *conn)
                .map_err(|e| AppError::EntityCreationError(e.to_string()))?;
            results.push(inserted);
        }

        Ok(results)
    }
}

impl Repository<Character, NewCharacter> for CharacterRepository {
    fn get_all(conn: &Arc<Mutex<SqliteConnection>>) -> Result<Vec<Character>, AppError> {
        let mut conn = Self::get_connection(conn)?;

        characters::table
            .select(Character::as_select())
            .load(&mut *conn)
            .map_err(|e| AppError::DatabaseOperationError(e.to_string()))
    }

    fn get_by_id(
        conn: &Arc<Mutex<SqliteConnection>>,
        character_id: i32,
    ) -> Result<Character, AppError> {
        let mut conn = Self::get_connection(conn)?;

        characters::table
            .find(character_id)
            .first(&mut *conn)
            .map_err(|_| {
                AppError::EntityNotFoundError(
                    "Could not find character with the provided ID in database.".to_string(),
                )
            })
    }

    fn insert(
        conn: &Arc<Mutex<SqliteConnection>>,
        character: NewCharacter,
    ) -> Result<Character, AppError> {
        let mut conn = Self::get_connection(conn)?;

        diesel::insert_into(characters::table)
            .values(character)
            .returning(Character::as_returning())
            .get_result(&mut *conn)
            .map_err(|_| {
                AppError::EntityCreationError(
                    "Could not insert new character into database.".to_string(),
                )
            })
    }

    fn update(conn: &Arc<Mutex<SqliteConnection>>, character: Character) -> Result<(), AppError> {
        let mut conn = Self::get_connection(conn)?;

        diesel::update(characters::table.filter(characters::id.eq(character.id)))
            .set(character)
            .execute(&mut *conn)
            .map_err(|_| {
                AppError::EntitySaveError("Failed to save character in database.".to_string())
            })?;

        Ok(())
    }

    fn delete(conn: &Arc<Mutex<SqliteConnection>>, entry_id: i32) -> Result<(), AppError> {
        let mut conn = Self::get_connection(conn)?;

        diesel::delete(characters::table.filter(characters::id.eq(entry_id)))
            .execute(&mut *conn)
            .map_err(|_| {
                AppError::EntityNotFoundError(
                    "Could not find and delete character in database.".to_string(),
                )
            })?;

        Ok(())
    }
}
