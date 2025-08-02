use std::sync::{Arc, Mutex};

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::{
    errors::AppError,
    models::character::{Character, NewCharacter},
    schema::characters,
    traits::repository::Repository,
};

pub struct CharacterRepository;

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
                AppError::DatabaseOperationError(
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
                AppError::DatabaseOperationError(
                    "Could not insert new character into database.".to_string(),
                )
            })
    }

    fn update(conn: &Arc<Mutex<SqliteConnection>>, entity: Character) -> Result<(), AppError> {
        todo!()
    }

    fn delete(conn: &Arc<Mutex<SqliteConnection>>, entry_id: i32) -> Result<(), AppError> {
        let mut conn = Self::get_connection(conn)?;

        diesel::delete(characters::table.filter(characters::id.eq(entry_id)))
            .execute(&mut *conn)
            .map_err(|_| {
                AppError::DatabaseOperationError(
                    "Could not find and delete character in database.".to_string(),
                )
            })?;

        Ok(())
    }
}
