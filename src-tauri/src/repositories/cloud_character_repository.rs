use futures::TryStreamExt;
use mongodb::Database;

use crate::dtos::character_dtos::FullCharacterData;
use crate::errors::AppError;

const COLLECTION_NAME: &str = "characters";

pub struct CloudCharacterRepository;

impl CloudCharacterRepository {
    pub async fn replace_all(
        db: &Database,
        characters: Vec<FullCharacterData>,
    ) -> Result<(), AppError> {
        let collection = db.collection::<FullCharacterData>(COLLECTION_NAME);

        collection.drop().await?;

        if !characters.is_empty() {
            collection.insert_many(characters).await?;
        }

        Ok(())
    }

    pub async fn get_all(db: &Database) -> Result<Vec<FullCharacterData>, AppError> {
        let collection = db.collection::<FullCharacterData>(COLLECTION_NAME);

        let cursor = collection.find(mongodb::bson::doc! {}).await?;
        let characters: Vec<FullCharacterData> = cursor.try_collect().await?;

        Ok(characters)
    }
}
