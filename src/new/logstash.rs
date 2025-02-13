use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

use crate::LogInfo;

use super::Format;

pub struct LogstashFormat;

impl Format for LogstashFormat {
    type Input = LogInfo;
    type Error = ();

    fn try_transform(&self, mut info: LogInfo) -> Result<Self::Input, Self::Error> {
        let mut logstash_object = json!({"@message": info.message});

        if let Some(Value::String(ts)) = info.meta.remove("timestamp") {
            logstash_object["@timestamp"] = json!(ts);
        }

        let mut fields = HashMap::new();
        fields.insert("level".to_string(), json!(info.level.clone()));

        for (key, value) in info.meta.iter() {
            fields.insert(key.clone(), value.clone());
        }

        logstash_object["@fields"] = json!(fields);

        info.message = logstash_object.to_string();
        Ok(info)
    }
}

pub fn logstash() -> LogstashFormat {
    LogstashFormat
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_logstash_format() {
        let logstash_format = LogstashFormat;

        let info = LogInfo::new("info", "Test message");
        let result = logstash_format.transform(info).unwrap();

        let parsed: Value = serde_json::from_str(&result.message).unwrap();
        assert!(parsed.get("@message").is_some());
        assert!(parsed.get("@fields").is_some());
        assert_eq!(parsed["@message"], "Test message");
        assert_eq!(parsed["@fields"]["level"], "info");
    }

    #[test]
    fn test_logstash_format_with_metadata() {
        let logstash_format = LogstashFormat;
        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));
        info.meta
            .insert("transaction_id".to_string(), json!("abcd1234"));

        let result = logstash_format.transform(info).unwrap();
        let parsed: Value = serde_json::from_str(&result.message).unwrap();

        assert_eq!(parsed["@fields"]["user_id"], "1234");
        assert_eq!(parsed["@fields"]["transaction_id"], "abcd1234");
        assert_eq!(parsed["@fields"]["level"], "info");
    }

    #[test]
    fn test_metadata_preservation() {
        let logstash_format = LogstashFormat;
        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));
        info.meta.insert("session_id".to_string(), json!("abcd"));

        let result = logstash_format.transform(info.clone()).unwrap();
        assert_eq!(result.meta.get("user_id").unwrap(), &json!("1234"));
        assert_eq!(result.meta.get("session_id").unwrap(), &json!("abcd"));

        let parsed: Value = serde_json::from_str(&result.message).unwrap();
        assert_eq!(parsed["@fields"]["user_id"], "1234");
        assert_eq!(parsed["@fields"]["session_id"], "abcd");
    }

    #[test]
    fn test_logstash_format_with_no_timestamp_in_meta() {
        let logstash_format = LogstashFormat;
        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));

        let result = logstash_format.transform(info).unwrap();
        let parsed: Value = serde_json::from_str(&result.message).unwrap();
        assert!(parsed.get("@timestamp").is_none());
        assert_eq!(parsed["@fields"]["user_id"], "1234");
    }
}
