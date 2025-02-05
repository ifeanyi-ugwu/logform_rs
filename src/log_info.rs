use std::collections::HashMap;

use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}

impl LogInfo {
    pub fn new<L: Into<String>, M: Into<String>>(level: L, message: M) -> Self {
        Self {
            level: level.into(),
            message: message.into(),
            meta: HashMap::new(),
        }
    }

    pub fn with_meta<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.meta.insert(key.into(), value.into());
        self
    }

    pub fn without_meta<K: Into<String>>(mut self, key: K) -> Self {
        self.meta.remove(&key.into());
        self
    }
}
