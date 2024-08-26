use crate::{Format, FormatOptions, LogInfo};
use std::sync::Arc;

pub fn combine(formats: Vec<Format>) -> Format {
    let combined = move |info: LogInfo, _opts: FormatOptions| {
        let mut obj = info;

        for format in &formats {
            //since options are internally merged during transform, no need to pass format_opts here
            // let format_opts = format.options.clone();
            obj = match format.transform(obj.clone(), None) {
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
    use super::*;
    use crate::{create_format, simple};
    use colored::*;
    use std::collections::HashMap;

    #[test]
    fn test_combine_formatters() {
        let colorizer = create_format(|mut info: LogInfo, opts: FormatOptions| {
            if let Some(opts) = opts {
                if opts.get("all").is_some() {
                    info.message = info.message.red().to_string(); // Example colorizer
                }
            }
            Some(info)
        });

        // Combine aligner and colorizer
        let combined_formatter = combine(vec![colorizer, simple()]);

        let info = LogInfo::new("info", "Test message").add_meta("key", "value");

        let opts = Some(HashMap::from([("all".to_string(), "true".to_string())]));

        let result = combined_formatter.transform(info, opts).unwrap();
        println!("Combined format result: {:?}", result.message);
    }
}
