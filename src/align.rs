use crate::{create_format, Format, FormatOptions, LogInfo};

pub fn align() -> Format {
    create_format(move |mut info: LogInfo, _options: FormatOptions| {
        // Add a tab character before the message
        info.message = format!("\t{}", info.message);
        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;

    #[test]
    fn test_align_format() {
        // Initialize the formatter
        let formatter = align();

        // Example log info
        let info = LogInfo::new("info", "Test message").add_meta("key", "value");

        // Apply the align formatter
        let result = formatter.transform(info, None).unwrap();
        println!("Aligned message: {}", result.message);

        // Verify that the message starts with a tab character
        assert!(result.message.starts_with('\t'));
        assert_eq!(result.message, "\tTest message");
    }
}
