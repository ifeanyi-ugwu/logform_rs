use crate::{create_format, Format, FormatOptions, LogInfo};

pub fn simple() -> Format {
    create_format(|info: LogInfo, _opts: FormatOptions| {
        // Get padding if present in meta, otherwise default to an empty string
        let padding = info
            .meta
            .get("padding")
            .and_then(|v| v.get(&info.level))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Start constructing the message with level, padding, and main message
        let mut message = format!("{}:{} {}", info.level, padding, info.message);

        // Clone the meta to filter out certain fields
        let mut rest = info.meta.clone();
        rest.remove("level");
        rest.remove("message");
        rest.remove("splat");
        rest.remove("padding"); // Remove the padding field

        // If there are remaining fields, stringify them and append to the message
        if !rest.is_empty() {
            let rest_string = serde_json::to_string(&rest).unwrap_or_default();
            message.push_str(&format!(" {}", rest_string));
        }

        // Return the new LogInfo with the constructed message
        Some(LogInfo {
            level: info.level,
            message,
            meta: info.meta,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use serde_json::json;

    #[test]
    fn test_simple_formatter() {
        let formatter = simple();

        let info = LogInfo::new("info", "User logged in")
            .add_meta("user_id", 12345)
            .add_meta("session_id", "abcde12345")
            .add_meta("padding", json!({"info": "    "}));

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);

        // Expected output:
        // info:    User logged in {"user_id":12345,"session_id":"abcde12345"}
    }
}

/*
pub struct SimpleFormat;

impl LogFormat for SimpleFormat {
    fn transform(&self, info: LogInfo, opts: Option<&HashMap<String, String>>) -> Option<LogInfo> {
        let padding = info
            .meta
            .get("padding")
            .and_then(|v| v.get(&info.level))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut message = format!("{}:{} {}", info.level, padding, info.message);

        let mut rest = info.meta.clone();
        rest.remove("level");
        rest.remove("message");
        rest.remove("splat");

        // Add the rest of the meta information if not empty
        if !rest.is_empty() {
            let rest_string = serde_json::to_string(&rest).unwrap_or_default();
            message.push_str(&format!(" {}", rest_string));
        }

        Some(LogInfo {
            level: info.level,
            message,
            meta: info.meta,
        })
    }
}
/*
pub fn simple() -> SimpleFormat {
    SimpleFormat
}
*/
pub fn simple() -> BoxedLogFormat {
    Box::new(SimpleFormat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::timestamp;
    use std::collections::HashMap;

    #[test]
    fn test_timestamp_and_simple_format() {
        // Create a LogInfo instance with some initial data
        let log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        // Apply SimpleFormat
        let timestamp_format = timestamp(None);
        let log_info = timestamp_format.transform(log_info, None).unwrap();

        // Apply JsonFormat
        let simple_format = simple();
        let log_info = simple_format.transform(log_info, None).unwrap();

        println!("{}", log_info.message);
    }
}
*/
