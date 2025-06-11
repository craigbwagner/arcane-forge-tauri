use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_uses: Option<u8>,
    pub uses_reset_on: Option<String>, // store enum as string
    pub action_type: Option<String>,   // store enum as string
    pub duration: Option<String>,
    pub feature_type: Option<String>,
    pub source: Option<String>,
    pub level_requirement: Option<u8>,
    pub is_passive: bool,
}
