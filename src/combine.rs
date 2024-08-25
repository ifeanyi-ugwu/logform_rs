use crate::{Format, FormatOptions, LogFormat, LogInfo};
use std::sync::Arc;

pub fn combine(formats: Vec<Format>) -> Format {
    let combined = move |info: LogInfo, opts: FormatOptions| {
        let mut obj = info;
        let opts = opts;
        for format in &formats {
            obj = match format.transform(obj.clone(), opts.clone()) {
                Some(new_info) => new_info,
                None => return None,
            };
        }
        Some(obj)
    };

    Format {
        format_fn: Arc::new(combined),
        options: None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{create_format, printf, simple};
    use colored::*;
    use serde_json::json;

    #[test]
    fn test_combine_formatters() {
        let aligner = create_format(|mut info: LogInfo, _opts: FormatOptions| {
            info.message = format!("\t{}", info.message);
            Some(info)
        });

        let colorizer = create_format(|mut info: LogInfo, opts: FormatOptions| {
            if let Some(opts) = opts {
                if opts.get("all").is_some() {
                    info.message = info.message.red().to_string(); // Example colorizer
                }
            }
            Some(info)
        });

        let formatter = printf(|info: &LogInfo| {
            format!(
                "{} - {}: {}",
                info.level,
                info.message,
                serde_json::to_string(&info.meta).unwrap_or_default()
            )
        });

        // Combine aligner and colorizer
        let combined_formatter = combine(vec![aligner, colorizer, formatter]);

        let mut meta = HashMap::new();
        meta.insert("key".to_string(), json!("value"));

        let info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta,
        };

        let opts = Some(HashMap::from([("all".to_string(), "true".to_string())]));

        let result = combined_formatter.transform(info, opts).unwrap();
        println!("Combined format result: {:?}", result.message);
    }
}

/*
pub struct CombineFormat {
    formats: Vec<BoxedLogFormat>,
}

impl CombineFormat {
    pub fn new(formats: Vec<BoxedLogFormat>) -> Self {
        Self { formats }
    }
}

impl LogFormat for CombineFormat {
    fn transform(&self, info: LogInfo, opts: Option<&HashMap<String, String>>) -> Option<LogInfo> {
        let mut current_info = info;

        for format in &self.formats {
            if let Some(new_info) = format.transform(current_info, None) {
                current_info = new_info;
            } else {
                return None;
            }
        }

        Some(current_info)
    }
}
/*
pub fn combine(formats: Vec<Box<dyn LogFormat>>) -> CombineFormat {
    CombineFormat::new(formats)
}
*/
pub fn combine(formats: Vec<BoxedLogFormat>) -> BoxedLogFormat {
    Box::new(CombineFormat::new(formats))
}
*/
