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

    pub fn meta<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        self.meta.get(key.as_ref())
    }

    pub fn meta_as_str<K: AsRef<str>>(&self, key: K) -> Option<&str> {
        self.meta(key).and_then(Value::as_str)
    }

    pub fn meta_as_bool<K: AsRef<str>>(&self, key: K) -> Option<bool> {
        self.meta(key).and_then(Value::as_bool)
    }

    pub fn meta_as_f64<K: AsRef<str>>(&self, key: K) -> Option<f64> {
        self.meta(key).and_then(Value::as_f64)
    }

    pub fn meta_as_object<K: AsRef<str>>(&self, key: K) -> Option<&Map<String, Value>> {
        self.meta(key).and_then(Value::as_object)
    }

    pub fn meta_as_array<K: AsRef<str>>(&self, key: K) -> Option<&Vec<Value>> {
        self.meta(key).and_then(Value::as_array)
    }
}
