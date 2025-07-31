use crate::LogInfo;

use super::{Format, FormatOptions};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub fn logstash() -> Format {
    Format::new(|mut info: LogInfo, _opts: FormatOptions| {
        // Create the Logstash-compatible JSON object
        let mut logstash_object = json!({});

        // Add the `@message` field
        logstash_object["@message"] = json!(info.message);
        //info.message = "".to_string(); // Clear the original message field

        // Add the `@timestamp` field, only if it exists in the meta
        if let Some(Value::String(ts)) = info.meta.remove("timestamp") {
            logstash_object["@timestamp"] = json!(ts);
        }

        // Create the `@fields` object
        let mut fields = HashMap::new();

        // Include the log level as a field
        fields.insert("level".to_string(), json!(info.level.clone()));

        // Add remaining metadata as fields
        for (key, value) in info.meta.iter() {
            fields.insert(key.clone(), value.clone());
        }

        // Add the fields to the logstash object
        logstash_object["@fields"] = json!(fields);

        // Convert the logstash object to a string and store it in `info.message`
        info.message = logstash_object.to_string();

        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use super::super::timestamp;
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_logstash_format() {
        let logstash_format = logstash();

        let info = LogInfo::new("info", "Test message");

        // Apply the timestamp format first
        let mut custom_opts = HashMap::new();
        custom_opts.insert("format".to_string(), "%+".to_string());
        let timestamp_format = timestamp();
        let info = timestamp_format.transform(info, Some(custom_opts)).unwrap();

        let result = logstash_format.transform(info, None).unwrap();

        // Parse the resulting message as JSON
        let parsed: Value = serde_json::from_str(&result.message).unwrap();

        // Verify the structure
        assert!(parsed.get("@message").is_some());
        assert!(parsed.get("@timestamp").is_some());
        assert!(parsed.get("@fields").is_some());

        // Verify specific fields
        assert_eq!(parsed["@message"], "Test message");
        assert_eq!(parsed["@fields"]["level"], "info");
    }

    #[test]
    fn test_logstash_format_with_metadata() {
        let logstash_format = logstash();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));
        info.meta
            .insert("transaction_id".to_string(), json!("abcd1234"));

        // Apply the timestamp format first
        let mut custom_opts = HashMap::new();
        custom_opts.insert("format".to_string(), "%+".to_string());
        let timestamp_format = timestamp();
        let info = timestamp_format.transform(info, Some(custom_opts)).unwrap();

        let result = logstash_format.transform(info, None).unwrap();

        // Parse the resulting message as JSON
        let parsed: Value = serde_json::from_str(&result.message).unwrap();

        // Verify the structure
        assert!(parsed.get("@message").is_some());
        assert!(parsed.get("@timestamp").is_some());
        assert!(parsed.get("@fields").is_some());

        // Verify specific fields in metadata
        let fields = parsed["@fields"].as_object().unwrap();
        assert_eq!(fields.get("user_id").unwrap(), "1234");
        assert_eq!(fields.get("transaction_id").unwrap(), "abcd1234");
        assert_eq!(fields.get("level").unwrap(), "info");
    }

    #[test]
    fn test_metadata_preservation() {
        let logstash_format = logstash();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));
        info.meta.insert("session_id".to_string(), json!("abcd"));

        let result = logstash_format.transform(info.clone(), None).unwrap();

        // Ensure original metadata is still in `info.meta`
        assert_eq!(result.meta.get("user_id").unwrap(), &json!("1234"));
        assert_eq!(result.meta.get("session_id").unwrap(), &json!("abcd"));

        // Ensure metadata was also copied to `@fields`
        let parsed: serde_json::Value = serde_json::from_str(&result.message).unwrap();
        assert_eq!(parsed["@fields"]["user_id"], "1234");
        assert_eq!(parsed["@fields"]["session_id"], "abcd");
    }

    #[test]
    fn test_logstash_format_with_no_timestamp_in_meta() {
        let logstash_format = logstash();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("user_id".to_string(), json!("1234"));

        let result = logstash_format.transform(info, None).unwrap();

        // Parse the resulting message as JSON
        let parsed: Value = serde_json::from_str(&result.message).unwrap();

        // Verify that timestamp was not added
        assert!(parsed.get("@timestamp").is_none());

        // Verify metadata was added to `@fields`
        let fields = parsed["@fields"].as_object().unwrap();
        assert_eq!(fields.get("user_id").unwrap(), "1234");
    }
}
