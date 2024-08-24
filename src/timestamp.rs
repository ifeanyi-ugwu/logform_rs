use crate::log_alt::{LogFormat, LogInfo};
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
    fn transform(&self, info: &mut LogInfo) {
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
    }
}
/*
pub fn timestamp(options: Option<TimestampOptions>) -> TimestampFormat {
    TimestampFormat::new(options)
}
*/
pub fn timestamp(options: Option<TimestampOptions>) -> Box<dyn LogFormat> {
    Box::new(TimestampFormat::new(options))
}
