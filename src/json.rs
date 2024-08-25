use crate::{create_format, log_alt::LogInfo, Format};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn json() -> Format {
    create_format(|info: LogInfo, _opts: Option<&HashMap<String, String>>| {
        // Create a JSON object including the level, message, and other meta data
        let mut log_object = Map::new();

        log_object.insert("level".to_string(), Value::String(info.level.clone()));
        log_object.insert("message".to_string(), Value::String(info.message.clone()));

        // Include other meta information
        for (key, value) in &info.meta {
            log_object.insert(key.clone(), value.clone());
        }

        // Convert the log object to a JSON string
        let json_message = Value::Object(log_object).to_string();

        // Return a new LogInfo object with the JSON message
        Some(LogInfo {
            level: info.level,
            message: json_message,
            meta: info.meta,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_json_formatter() {
        let formatter = json();

        let mut meta = HashMap::new();
        meta.insert("user_id".to_string(), json!(12345));
        meta.insert("session_id".to_string(), json!("abcde12345"));

        let info = LogInfo {
            level: "info".to_string(),
            message: "User logged in".to_string(),
            meta,
        };

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);

        // The output should be a JSON string like:
        // {"level":"info","message":"User logged in","user_id":12345,"session_id":"abcde12345"}
    }
}

/*
pub struct JsonFormat;

impl LogFormat for JsonFormat {
    fn transform(&self, info: LogInfo, opts: Option<&HashMap<String, String>>) -> Option<LogInfo> {
        // Create a JSON object including the level, message, and other meta data
        let mut log_object = serde_json::Map::new();

        log_object.insert("level".to_string(), Value::String(info.level.clone()));
        log_object.insert("message".to_string(), Value::String(info.message.clone()));

        // Include other meta information
        for (key, value) in &info.meta {
            log_object.insert(key.clone(), value.clone());
        }

        // Convert the log object to a JSON string
        let json_message = Value::Object(log_object).to_string();

        // Return a new LogInfo object with the JSON message
        Some(LogInfo {
            level: info.level,
            message: json_message,
            meta: info.meta,
        })
    }
}
/*
pub fn json() -> JsonFormat {
    JsonFormat
}
*/
pub fn json() -> BoxedLogFormat {
    Box::new(JsonFormat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::timestamp_builder;
    use std::collections::HashMap;

    #[test]
    fn test_timestamp_and_json_format() {
        // Create a LogInfo instance with some initial data
        let log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        let timestamp_format = timestamp_builder()
            .format("%Y-%m-%d %H:%M:%S")
            .alias("time")
            .build();

        // Apply TimestampFormat
        let log_info = timestamp_format.transform(log_info, None).unwrap();

        // Apply JsonFormat
        let json_format = json();
        let log_info = json_format.transform(log_info, None).unwrap();

        println!("{}", log_info.message);
    }
}
*/
