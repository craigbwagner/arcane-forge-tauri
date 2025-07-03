use crate::errors::AppError;

pub trait Repository<T> {
    fn get_all(&self) -> Result<Vec<T>, AppError>;
    fn get_by_id(&self, id: i32) -> Result<Option<T>, AppError>;
    fn insert(&self, entity: T) -> Result<i32, AppError>;
    fn update(&self, entity: T) -> Result<(), AppError>;
    fn delete(&self, id: i32) -> Result<(), AppError>;
}
