use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub subclass: String,
    pub level: u8,
    pub class_feature_ids: Vec<i64>,
    pub subclass_ids: Vec<i64>,
}
