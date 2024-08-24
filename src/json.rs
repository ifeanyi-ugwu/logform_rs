use crate::log_alt::{BoxedLogFormat, LogFormat, LogInfo};
use serde_json::Value;

pub struct JsonFormat;

impl LogFormat for JsonFormat {
    fn transform(&self, info: &mut LogInfo) {
        // Create a JSON object including the level, message, and other meta data
        let mut log_object = serde_json::Map::new();

        log_object.insert("level".to_string(), Value::String(info.level.clone()));
        log_object.insert("message".to_string(), Value::String(info.message.clone()));

        // Include other meta information
        for (key, value) in &info.meta {
            log_object.insert(key.clone(), value.clone());
        }

        // Convert the log object to a JSON string
        info.message = Value::Object(log_object).to_string();
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
    use crate::timestamp::{timestamp, TimestampOptions};
    use std::collections::HashMap;

    #[test]
    fn test_timestamp_and_json_format() {
        // Create a LogInfo instance with some initial data
        let mut log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        let timestamp_options = TimestampOptions {
            format: Some("%Y-%m-%d %H:%M:%S".to_string()),
            alias: Some("time".to_string()),
        };

        // Apply TimestampFormat
        let timestamp_format = timestamp(Some(timestamp_options));
        timestamp_format.transform(&mut log_info);

        // Apply JsonFormat
        let json_format = json();
        json_format.transform(&mut log_info);

        println!("{}", log_info.message);
    }
}
