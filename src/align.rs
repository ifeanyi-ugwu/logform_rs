use crate::{create_format, Format, LogInfo};
use std::collections::HashMap;

pub fn align_format() -> Format {
    create_format(
        move |mut info: LogInfo, _options: Option<&HashMap<String, String>>| {
            // Add a tab character before the message
            info.message = format!("\t{}", info.message);
            Some(info)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_align_format() {
        // Initialize the formatter
        let formatter = align_format();

        // Example log info
        let mut meta = HashMap::new();
        meta.insert("key".to_string(), json!("value"));

        let info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta,
        };

        // Apply the align formatter
        let result = formatter.transform(info, None).unwrap();
        println!("Aligned message: {}", result.message);

        // Verify that the message starts with a tab character
        assert!(result.message.starts_with('\t'));
        assert_eq!(result.message, "\tTest message");
    }
}
