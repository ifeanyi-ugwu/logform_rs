use crate::{create_format, Format, FormatOptions, LogInfo};
use regex::Regex;
use std::collections::HashMap;

pub fn uncolorize() -> Format {
    create_format(move |mut info: LogInfo, options: FormatOptions| {
        let binding = HashMap::new();
        let opts = options.unwrap_or(binding);

        if opts.get("level").unwrap_or(&"true".to_string()) != "false" {
            info.level = strip_colors(&info.level);
        }

        if opts.get("message").unwrap_or(&"true".to_string()) != "false" {
            info.message = strip_colors(&info.message);
        }

        Some(info)
    })
}

fn strip_colors(input: &str) -> String {
    // Regex pattern to match ANSI escape codes
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(input, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colorize;
    use colored::control::set_override;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_uncolorize_formatter() {
        // Force colored output even if not in a TTY environment
        set_override(true);

        // Step 1: Colorize
        let mut opts = HashMap::new();
        opts.insert("all".to_string(), "true".to_string());
        opts.insert(
            "colors".to_string(),
            json!({
                "info": ["blue"],
                "error": ["red", "bold"]
            })
            .to_string(),
        );
        let colorizer = colorize(Some(opts));

        let info = LogInfo::new("info", "This is an info message").add_meta("key", "value");

        let opts = Some(HashMap::from([("all".to_string(), "true".to_string())]));

        let colorized_info = colorizer.transform(info, opts).unwrap();

        // Step 2: Uncolorize
        let uncolorizer = uncolorize();
        let uncolorized_info = uncolorizer.transform(colorized_info.clone(), None).unwrap();

        // Print for verification
        println!("Colorized level: {}", colorized_info.level);
        println!("Colorized message: {}", colorized_info.message);
        println!("Uncolored level: {}", uncolorized_info.level);
        println!("Uncolored message: {}", uncolorized_info.message);

        // Assertions
        assert_eq!(strip_colors(&colorized_info.level), "info");
        assert_eq!(
            strip_colors(&colorized_info.message),
            "This is an info message"
        );
    }
}
