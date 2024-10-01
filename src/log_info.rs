use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: Map<String, Value>,
}

impl LogInfo {
    pub fn new<S: Into<String>>(level: S, message: S) -> Self {
        Self {
            level: level.into(),
            message: message.into(),
            meta: Map::new(), //really not needed. no needed scenario for it to be a map instead of hashmap
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

    pub fn remove_meta<K: Into<String>>(mut self, key: K) -> Self {
        self.meta.remove(&key.into());
        self
    }

    pub fn get_meta<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        self.meta.get(key.as_ref())
    }

    pub fn get_meta_string<K: AsRef<str>>(&self, key: K) -> Option<&str> {
        self.get_meta(key).and_then(Value::as_str)
    }

    pub fn get_meta_bool<K: AsRef<str>>(&self, key: K) -> Option<bool> {
        self.get_meta(key).and_then(Value::as_bool)
    }

    pub fn get_meta_number<K: AsRef<str>>(&self, key: K) -> Option<f64> {
        self.get_meta(key).and_then(Value::as_f64)
    }

    pub fn get_meta_object<K: AsRef<str>>(&self, key: K) -> Option<&Map<String, Value>> {
        self.get_meta(key).and_then(Value::as_object)
    }

    pub fn get_meta_array<K: AsRef<str>>(&self, key: K) -> Option<&Vec<Value>> {
        self.get_meta(key).and_then(Value::as_array)
    }
}
