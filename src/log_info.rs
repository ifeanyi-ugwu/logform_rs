use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}

impl LogInfo {
    /*TODO check to remove or remove the builder so those setter methods are
    in the LogInfo directly. Choose a cleaner arch
    or even keep the both, so the user can either build or use new

    considerations to check
    since the new or the LogInfo does not depend on being built before logged
    we can just  use the new and add teh method on on it
    so that a user can just use it with new and build on the fly
    find out how the log::Record crate did theirs
    */

    pub fn new(level: &str, message: &str) -> Self {
        Self {
            level: level.to_string(),
            message: message.to_string(),
            meta: HashMap::new(),
        }
    }

    pub fn builder(level: &str, message: &str) -> LogInfoBuilder {
        LogInfoBuilder::new(level, message)
    }
}

pub struct LogInfoBuilder {
    level: String,
    message: String,
    meta: HashMap<String, Value>,
}

impl LogInfoBuilder {
    pub fn new(level: &str, message: &str) -> Self {
        Self {
            level: level.to_string(),
            message: message.to_string(),
            meta: HashMap::new(),
        }
    }

    pub fn level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn meta(mut self, key: &str, value: Value) -> Self {
        self.meta.insert(key.to_string(), value);
        self
    }

    pub fn build(self) -> LogInfo {
        LogInfo {
            level: self.level,
            message: self.message,
            meta: self.meta,
        }
    }
}
