use crate::repositories::{
    character_repository::CharacterRepository, item_repository::ItemRepository,
};

pub struct AppState {
    pub character_repo: CharacterRepository,
    pub item_repo: ItemRepository,
}
