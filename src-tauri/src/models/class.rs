use serde::{Deserialize, Serialize};

use crate::models::feature::Feature;
use crate::models::subclass::Subclass;

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub level: u8,
    pub class_features: Vec<Feature>,
    pub subclasses: Vec<Subclass>,
}
