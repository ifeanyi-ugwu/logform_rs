use crate::{Format, FormatOptions, LogInfo};
use serde_json::{Map, Value};

pub fn json() -> Format {
    Format::new(|info: LogInfo, _opts: FormatOptions| {
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

    #[test]
    fn test_json_formatter() {
        let formatter = json();

        let info = LogInfo::new("info", "User logged in")
            .add_meta("user_id", 12345)
            .add_meta("session_id", "abcde12345");

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);

        // The output should be a JSON string like:
        // {"level":"info","message":"User logged in","user_id":12345,"session_id":"abcde12345"}
    }
}
