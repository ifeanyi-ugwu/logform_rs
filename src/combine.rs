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
    use crate::{simple, timestamp};

    #[test]
    fn test_combine_formatters() {
        // Combine timestamp and simple
        let combined_formatter = combine(vec![timestamp(), simple()]);

        let info = LogInfo::new("info", "Test message").add_meta("key", "value");

        let result = combined_formatter.transform(info, None).unwrap();
        println!("{}", result.message);
    }
}
