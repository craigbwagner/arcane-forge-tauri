use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

pub struct AppState {
    pub db: Arc<Mutex<SqliteConnection>>,
}
