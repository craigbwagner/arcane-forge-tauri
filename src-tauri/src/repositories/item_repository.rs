use crate::{db::Database, errors::AppError, models::item::Item, traits::repository::Repository};

pub struct ItemRepository {
    db: Database,
}

impl Repository<Item> for ItemRepository {
    fn get_all(&self) -> std::result::Result<Vec<Item>, AppError> {
        todo!()
    }

    fn get_by_id(&self, id: i32) -> std::result::Result<Option<Item>, AppError> {
        todo!()
    }

    fn insert(&self, entity: Item) -> std::result::Result<i32, AppError> {
        todo!()
    }

    fn update(&self, entity: Item) -> std::result::Result<(), AppError> {
        todo!()
    }

    fn delete(&self, id: i32) -> std::result::Result<(), AppError> {
        todo!()
    }
}

impl ItemRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}
