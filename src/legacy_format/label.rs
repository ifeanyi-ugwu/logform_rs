use serde_json::json;

use super::{Format, FormatOptions};
use crate::LogInfo;

pub fn label() -> Format {
    Format::new(|mut info: LogInfo, opts: FormatOptions| {
        // Get the label from options (or default to an empty string)
        let label = opts
            .clone()
            .and_then(|o| o.get("label").cloned())
            .map(|v| v.to_string())
            .unwrap_or_else(|| "".to_string());

        // Check if the label should be added to the message or to the info.label
        let message = opts
            .and_then(|o| o.get("message").cloned())
            .map(|v| v == "true")
            .unwrap_or(true);

        if message {
            // Add label before the message
            info.message = format!("[{}] {}", label, info.message);
        } else {
            // Add label to info object
            info.meta.insert("label".to_string(), json!(label));
        }

        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_label_format_message() {
        let label_format = label();

        let info = LogInfo::new("info", "Test message");

        let mut opts = HashMap::new();
        opts.insert("label".to_string(), "MY_LABEL".to_string());
        opts.insert("message".to_string(), "true".to_string());

        let result = label_format.transform(info, Some(opts)).unwrap();
        println!("{:?}", result);
        assert_eq!(result.message, "[MY_LABEL] Test message");
    }

    #[test]
    fn test_label_format_meta() {
        let label_format = label();

        let info = LogInfo::new("info", "Test message");

        let mut opts = HashMap::new();
        opts.insert("label".to_string(), "MY_LABEL".to_string());
        opts.insert("message".to_string(), "false".to_string());

        let result = label_format.transform(info, Some(opts)).unwrap();
        println!("{:?}", result);
        assert_eq!(result.meta.get("label"), Some(&json!("MY_LABEL")));
    }
}
