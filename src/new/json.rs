use super::Format;
use crate::LogInfo;
use serde_json::{Map, Value};

pub struct JsonFormat;

impl Format for JsonFormat {
    type Input = LogInfo;

    fn transform(&self, info: LogInfo) -> Option<Self::Input> {
        let mut log_object = Map::new();

        log_object.insert("level".to_string(), Value::String(info.level.clone()));
        log_object.insert("message".to_string(), Value::String(info.message.clone()));

        for (key, value) in &info.meta {
            log_object.insert(key.clone(), value.clone());
        }

        let json_message = Value::Object(log_object).to_string();

        Some(LogInfo {
            level: info.level,
            message: json_message,
            meta: info.meta,
        })
    }
}

pub fn json() -> JsonFormat {
    JsonFormat
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_json_format() {
        let json_formatter = JsonFormat;

        let info = LogInfo::new("info", "User logged in")
            .with_meta("user_id", Value::Number(12345.into()))
            .with_meta("session_id", Value::String("abcde12345".to_string()));

        let result = json_formatter.transform(info).unwrap();
        let expected_json = json!({
            "level": "info",
            "message": "User logged in",
            "user_id": 12345,
            "session_id": "abcde12345"
        })
        .to_string();

        assert_eq!(result.message, expected_json);
    }
}
