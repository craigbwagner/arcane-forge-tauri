use serde::{Deserialize, Serialize};

use crate::models::{ActionType, UseReset};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub id: i64,                 // SQLite INTEGER PRIMARY KEY AUTOINCREMENT
    pub character_ids: Vec<i64>, // FK to Character.id's
    pub name: Option<String>,
    pub description: Option<String>,
    pub uses: Option<u8>,
    pub uses_reset_on: Option<UseReset>, // store enum as string
    pub action_type: Option<ActionType>, // store enum as string
    pub duration: Option<String>,
    pub source: Option<String>,
    pub level_acquired: Option<u8>,
    pub is_passive: bool,
}
