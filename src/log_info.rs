#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use std::io::Result as IoResult;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// Convert LogInfo to JSON bytes
    #[cfg(feature = "serde")]
    pub fn to_bytes(&self) -> IoResult<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Convert JSON bytes to LogInfo
    #[cfg(feature = "serde")]
    pub fn from_bytes(bytes: &[u8]) -> IoResult<Self> {
        serde_json::from_slice(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Convert serde_json::Value to LogInfo
    pub fn from_value(value: Value) -> Result<Self, String> {
        if let Value::Object(map) = value {
            let level = map
                .get("level")
                .and_then(Value::as_str)
                .ok_or("Missing or invalid 'level' field")?
                .to_string();

            let message = map
                .get("message")
                .and_then(Value::as_str)
                .ok_or("Missing or invalid 'message' field")?
                .to_string();

            let mut meta = HashMap::new();
            if let Some(meta_value) = map.get("meta") {
                if let Value::Object(meta_map) = meta_value.clone() {
                    for (key, value) in meta_map {
                        meta.insert(key, value);
                    }
                }
            }

            Ok(Self {
                level,
                message,
                meta,
            })
        } else {
            Err("Input value is not a JSON object".to_string())
        }
    }

    pub fn to_value(&self) -> Value {
        serde_json::json!({
            "level": self.level,
            "message": self.message,
            "meta": self.meta,
        })
    }
}

#[macro_export]
macro_rules! log_info {
    // Without metadata
    ($level:ident, $msg:expr) => {{
        $crate::LogInfo::new(stringify!($level), $msg)
    }};

    // With metadata
    ($level:ident, $msg:expr, $($key:ident = $value:expr),*) => {{
        let mut log_entry = $crate::LogInfo::new(stringify!($level), $msg);
        $(
            log_entry = log_entry.with_meta(stringify!($key), $value);
        )*
        log_entry
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[cfg(feature = "serde")]
    #[test]
    fn test_byte_serialization_and_deserialization() {
        let log = LogInfo::new("INFO", "Test message")
            .with_meta("user", "Alice")
            .with_meta("attempts", 3);

        let json_bytes = log.to_bytes().expect("Failed to serialize to JSON");
        let deserialized_log =
            LogInfo::from_bytes(&json_bytes).expect("Failed to deserialize JSON");

        assert_eq!(deserialized_log.level, "INFO");
        assert_eq!(deserialized_log.message, "Test message");
        assert_eq!(deserialized_log.meta["user"], json!("Alice"));
        assert_eq!(deserialized_log.meta["attempts"], json!(3));
    }

    #[test]
    fn test_from_value() {
        let json_value = json!({
            "level": "DEBUG",
            "message": "Another test message",
            "meta": {
                "id": 12345,
                "status": "pending"
            }
        });

        let log_info =
            LogInfo::from_value(json_value).expect("Failed to create LogInfo from Value");

        assert_eq!(log_info.level, "DEBUG");
        assert_eq!(log_info.message, "Another test message");
        assert_eq!(log_info.meta["id"], json!(12345));
        assert_eq!(log_info.meta["status"], json!("pending"));
    }
}
