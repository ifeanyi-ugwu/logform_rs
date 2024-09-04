use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}

impl LogInfo {
    pub fn new<S: Into<String>>(level: S, message: S) -> Self {
        Self {
            level: level.into(),
            message: message.into(),
            meta: HashMap::new(),
        }
    }

    pub fn add_meta<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.meta.insert(key.into(), value.into());
        self
    }

    pub fn remove_meta<K>(mut self, key: K) -> Self
    where
        K: Into<String>,
    {
        self.meta.remove(&key.into());
        self
    }

    pub fn get_meta<K>(&self, key: K) -> Option<&Value>
    where
        K: Into<String>,
    {
        self.meta.get(&key.into())
    }
}
