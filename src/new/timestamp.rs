use super::Format;
use crate::LogInfo;
use chrono::{DateTime, Utc};
use serde_json::json;

#[derive(Clone)]
pub struct Timestamp {
    format: Option<String>,
    alias: Option<String>,
}

impl Timestamp {
    pub fn new(format: Option<String>, alias: Option<String>) -> Self {
        Timestamp { format, alias }
    }

    pub fn transform(&self, mut info: LogInfo) -> Option<LogInfo> {
        let timestamp = if let Some(fmt) = &self.format {
            let now: DateTime<Utc> = Utc::now();
            now.format(fmt).to_string()
        } else {
            Utc::now().to_rfc3339()
        };

        // Always set the timestamp field
        info.meta
            .insert("timestamp".to_string(), json!(timestamp.clone()));

        // Set alias if provided
        if let Some(alias) = &self.alias {
            info.meta.insert(alias.clone(), json!(timestamp));
        }

        Some(info)
    }
}

impl Format for Timestamp {
    type Input = LogInfo;
    type Error = (); // No actual error handling needed

    fn try_transform(&self, info: LogInfo) -> Result<Self::Input, Self::Error> {
        self.transform(info).ok_or(())
    }
}

pub fn timestamp(format: Option<String>, alias: Option<String>) -> Timestamp {
    Timestamp::new(format, alias)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_default_timestamp() {
        let formatter = timestamp(None, None);
        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info).unwrap();

        assert!(result.meta.contains_key("timestamp"));
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        println!("{}", timestamp);

        let rfc3339_regex =
            Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{6}([+-]\d{2}:\d{2}|Z)$").unwrap();
        assert!(rfc3339_regex.is_match(timestamp));
    }

    #[test]
    fn test_custom_format() {
        let formatter = timestamp(Some("%d/%m/%Y %H:%M:%S".to_string()), None);
        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info).unwrap();

        assert!(result.meta.contains_key("timestamp"));
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        let custom_format_regex = Regex::new(r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(custom_format_regex.is_match(timestamp));
    }

    #[test]
    fn test_alias() {
        let formatter = timestamp(None, Some("log_time".to_string()));
        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info).unwrap();

        assert!(result.meta.contains_key("timestamp"));
        assert!(result.meta.contains_key("log_time"));
        assert_eq!(result.meta.get("timestamp"), result.meta.get("log_time"));
    }

    #[test]
    fn test_custom_format_with_alias() {
        let formatter = timestamp(
            Some("%d/%m/%Y %H:%M:%S".to_string()),
            Some("log_time".to_string()),
        );
        let info = LogInfo::new("info", "Test message");
        let result = formatter.transform(info).unwrap();

        assert!(result.meta.contains_key("timestamp"));
        assert!(result.meta.contains_key("log_time"));
        let timestamp = result.meta.get("timestamp").unwrap().as_str().unwrap();
        let log_time = result.meta.get("log_time").unwrap().as_str().unwrap();

        assert_eq!(timestamp, log_time);
        let custom_format_regex = Regex::new(r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(custom_format_regex.is_match(timestamp));
    }
}
