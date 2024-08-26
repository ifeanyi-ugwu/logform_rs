use crate::{create_format, Format, FormatOptions, LogInfo};
use chrono::{DateTime, Utc};
use serde_json::json;

pub fn timestamp() -> Format {
    create_format(|mut info: LogInfo, opts: FormatOptions| {
        let format = opts
            .as_ref()
            .and_then(|o| o.get("format"))
            .map(String::as_str)
            .unwrap_or("%Y-%m-%d %H:%M:%S");

        let alias = opts
            .as_ref()
            .and_then(|o| o.get("alias"))
            .map(String::as_str)
            .unwrap_or("timestamp");

        let now: DateTime<Utc> = Utc::now();
        let timestamp = now.format(&format).to_string();

        info.meta.insert(alias.to_string(), json!(timestamp));
        Some(info)
    })
}
/*
pub struct TimestampBuilder {
    format: Option<String>,
    alias: Option<String>,
}

impl TimestampBuilder {
    pub fn new() -> Self {
        Self {
            format: None,
            alias: None,
        }
    }

    pub fn format(mut self, fmt: &str) -> Self {
        self.format = Some(fmt.to_string());
        self
    }

    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn build(self) -> BoxedLogFormat {
        let options = TimestampOptions {
            format: self.format,
            alias: self.alias,
        };
        Box::new(TimestampFormat::new(Some(options)))
    }
}

pub fn timestamp_builder() -> TimestampBuilder {
    TimestampBuilder::new()
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use std::collections::HashMap;

    #[test]
    fn test_timestamp_format() {
        let formatter = timestamp();

        let info = LogInfo::new("info", "This is a log message");

        let result = formatter.transform(info, None).unwrap();
        println!("{:?}", result.meta);

        let mut custom_opts = HashMap::new();
        custom_opts.insert("format".to_string(), "%d/%m/%Y %H:%M:%S".to_string());
        custom_opts.insert("alias".to_string(), "log_time".to_string());

        let info2 = LogInfo::new("info", "Another log message");

        let result2 = formatter.transform(info2, Some(custom_opts)).unwrap();
        println!("{:?}", result2.meta);
    }
}
