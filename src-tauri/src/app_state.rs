use std::sync::{Arc, Mutex};

use diesel::SqliteConnection;

pub struct AppState {
    db: Arc<Mutex<SqliteConnection>>,
}
