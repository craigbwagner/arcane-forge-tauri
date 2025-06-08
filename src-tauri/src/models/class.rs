use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub subclass: String,
    pub level: u8,
    pub class_features: Vec<i64>,
    pub subclass_features: Vec<i64>,
}

impl Default for Class {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            subclass: String::new(),
            level: 1,
            class_features: Vec::new(),
            subclass_features: Vec::new(),
        }
    }
}
