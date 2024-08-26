use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}

impl LogInfo {
    pub fn new(level: &str, message: &str) -> Self {
        Self {
            level: level.to_string(),
            message: message.to_string(),
            meta: HashMap::new(),
        }
    }

    pub fn add_meta<V>(mut self, key: &str, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.meta.insert(key.to_string(), value.into());
        self
    }

    pub fn remove_meta(mut self, key: &str) -> Self {
        self.meta.remove(key);
        self
    }

    pub fn get_meta(&self, key: &str) -> Option<&Value> {
        self.meta.get(key)
    }
}
