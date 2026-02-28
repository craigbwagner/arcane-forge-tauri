use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;
use mongodb::Database;

pub struct AppState {
    pub db: Arc<Mutex<SqliteConnection>>,
    pub mongo: Option<Database>,
}
