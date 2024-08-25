use crate::log_alt::{BoxedLogFormat, LogFormat, LogInfo};
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct TimestampOptions {
    pub format: Option<String>,
    pub alias: Option<String>,
}

pub struct TimestampFormat {
    options: Option<TimestampOptions>,
}

impl TimestampFormat {
    pub fn new(options: Option<TimestampOptions>) -> Self {
        Self { options }
    }

    fn format_timestamp(&self) -> String {
        match &self.options {
            Some(opts) => match &opts.format {
                Some(fmt) => {
                    // Format using the provided format
                    let now: DateTime<Utc> = Utc::now();
                    // Note: You'll need a library to format dates in custom formats
                    // Example: use `chrono::format::strftime(fmt)` for custom formatting
                    now.format(fmt).to_string()
                }
                None => Utc::now().to_rfc3339(),
            },
            None => Utc::now().to_rfc3339(),
        }
    }
}

impl LogFormat for TimestampFormat {
    fn transform(&self, mut info: LogInfo) -> Option<LogInfo> {
        let timestamp = self.format_timestamp();

        match &self.options {
            Some(opts) => {
                if let Some(alias) = &opts.alias {
                    info.meta.insert(alias.clone(), json!(timestamp));
                } else {
                    info.meta.insert("timestamp".to_string(), json!(timestamp));
                }
            }
            None => {
                info.meta.insert("timestamp".to_string(), json!(timestamp));
            }
        }
        Some(info)
    }
}
/*
pub fn timestamp(options: Option<TimestampOptions>) -> TimestampFormat {
    TimestampFormat::new(options)
}
*/
pub fn timestamp(options: Option<TimestampOptions>) -> BoxedLogFormat {
    Box::new(TimestampFormat::new(options))
}

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
