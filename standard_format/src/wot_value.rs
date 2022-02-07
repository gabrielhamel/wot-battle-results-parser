use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WotValue {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    Text(String),
    Collection(Vec<WotValue>),
    NamedCollection(HashMap<String, WotValue>),
    NamedIntCollection(HashMap<i64, WotValue>),
    OutOfBounds,
    NotAllowed,
}

impl Default for WotValue {
    fn default() -> Self { WotValue::None }
}