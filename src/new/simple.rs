use super::Format;
use crate::LogInfo;

pub struct SimpleFormat;

impl Format for SimpleFormat {
    type Input = LogInfo;
    type Error = (); // No real errors, so we use an empty tuple

    fn try_transform(&self, info: LogInfo) -> Result<Self::Input, Self::Error> {
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
        rest.remove("padding");

        if !rest.is_empty() {
            let rest_string = serde_json::to_string(&rest).unwrap_or_default();
            message.push_str(&format!(" {}", rest_string));
        }

        Ok(LogInfo {
            level: info.level,
            message,
            meta: info.meta,
        })
    }
}

pub fn simple() -> SimpleFormat {
    SimpleFormat
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_simple_format() {
        let simple_formatter = SimpleFormat;

        let info = LogInfo::new("info", "User logged in")
            .with_meta("user_id", Value::Number(12345.into()))
            .with_meta("session_id", Value::String("abcde12345".to_string()))
            .with_meta("padding", json!({"info": "    "}));

        let result = simple_formatter.transform(info).unwrap();

        // Split the expected message and metadata for separate assertions
        let expected_prefix = "info:     User logged in ";
        assert!(result.message.starts_with(expected_prefix));

        // Extract and parse the JSON part
        let json_part = result.message.strip_prefix(expected_prefix).unwrap();
        let actual_json: Value = serde_json::from_str(json_part).unwrap();

        // Expected JSON object
        let expected_json = json!({
            "user_id": 12345,
            "session_id": "abcde12345"
        });

        // Compare parsed JSON structures (ignores key order)
        assert_eq!(actual_json, expected_json);
    }
}
