use crate::{Format, FormatOptions, LogInfo};
use chrono::{DateTime, Utc};
use serde_json::json;

pub fn timestamp() -> Format {
    Format::new(|mut info: LogInfo, opts: FormatOptions| {
        let timestamp = opts
            .as_ref()
            .and_then(|o| o.get("format"))
            .map(String::as_str)
            .map(|fmt| {
                let now: DateTime<Utc> = Utc::now();
                now.format(fmt).to_string()
            })
            .unwrap_or_else(|| Utc::now().to_rfc3339());

        // Always set the timestamp field
        info.meta
            .insert("timestamp".to_string(), json!(timestamp.clone()));

        // Set alias if provided
        opts.as_ref()
            .and_then(|o| o.get("alias"))
            .map(String::as_str)
            .map(|alias| {
                info.meta.insert(alias.to_string(), json!(timestamp));
            });

        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_default_timestamp() {
        let formatter = timestamp();
        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info, None).unwrap();

        // Should always have timestamp field
        assert!(result.meta.contains_key("timestamp"));

        // Should be in RFC3339 format
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        println!("{}", timestamp);
        let rfc3339_regex =
            Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{6}([+-]\d{2}:\d{2}|Z)$").unwrap();
        assert!(rfc3339_regex.is_match(timestamp));
    }

    #[test]
    fn test_custom_format() {
        let formatter = timestamp();
        let mut custom_opts = HashMap::new();
        custom_opts.insert("format".to_string(), "%d/%m/%Y %H:%M:%S".to_string());

        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info, Some(custom_opts)).unwrap();

        // Should still have timestamp field
        assert!(result.meta.contains_key("timestamp"));

        // Should match custom format
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        let custom_format_regex = Regex::new(r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(custom_format_regex.is_match(timestamp));
    }

    #[test]
    fn test_alias() {
        let formatter = timestamp();
        let mut custom_opts = HashMap::new();
        custom_opts.insert("alias".to_string(), "log_time".to_string());

        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info, Some(custom_opts)).unwrap();

        // Should have both fields
        assert!(result.meta.contains_key("timestamp"));
        assert!(result.meta.contains_key("log_time"));

        // Both fields should have the same value
        assert_eq!(result.meta.get("timestamp"), result.meta.get("log_time"));
    }

    #[test]
    fn test_custom_format_with_alias() {
        let formatter = timestamp();
        let mut custom_opts = HashMap::new();
        custom_opts.insert("format".to_string(), "%d/%m/%Y %H:%M:%S".to_string());
        custom_opts.insert("alias".to_string(), "log_time".to_string());

        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info, Some(custom_opts)).unwrap();

        // Should have both fields
        assert!(result.meta.contains_key("timestamp"));
        assert!(result.meta.contains_key("log_time"));

        // Both fields should have the same value and match custom format
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        let log_time = result.meta.get("log_time").unwrap().as_str().unwrap();

        assert_eq!(timestamp, log_time);
        let custom_format_regex = Regex::new(r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(custom_format_regex.is_match(timestamp));
    }
}
