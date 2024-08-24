use serde_json::Value;
use std::collections::HashMap;

pub trait LogFormat {
    fn transform(&self, info: &mut LogInfo);
}

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
}
