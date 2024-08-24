use crate::log_alt::{LogFormat, LogInfo};

pub struct SimpleFormat;

impl LogFormat for SimpleFormat {
    fn transform(&self, info: &mut LogInfo) {
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

        info.message = message;
    }
}

pub fn simple() -> SimpleFormat {
    SimpleFormat
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::timestamp;
    use std::collections::HashMap;

    #[test]
    fn test_timestamp_and_simple_format() {
        // Create a LogInfo instance with some initial data
        let mut log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        // Apply SimpleFormat
        let timestamp_format = timestamp(None);
        timestamp_format.transform(&mut log_info);

        // Apply JsonFormat
        let simple_format = simple();
        simple_format.transform(&mut log_info);

        println!("{}", log_info.message);
    }
}
