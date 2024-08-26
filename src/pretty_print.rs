use crate::{create_format, Format, FormatOptions, LogInfo};
use serde_json::to_string_pretty;

pub fn pretty_print() -> Format {
    create_format(|info: LogInfo, opts: FormatOptions| {
        // Clone the meta to work with
        let mut meta = info.meta.clone();

        // Remove fields equivalent to LEVEL, MESSAGE, SPLAT
        meta.remove("level");
        meta.remove("message");
        meta.remove("splat");

        // Determine if we should colorize the output (default to false)
        let colorize = opts
            .as_ref()
            .and_then(|o| o.get("colorize"))
            .map_or(false, |v| v == "true");

        // Use serde_json to pretty print the remaining meta information
        let pretty_message = if colorize {
            // Apply color formatting here if needed (for now, this just uses normal printing)
            to_string_pretty(&meta).unwrap_or_default()
        } else {
            to_string_pretty(&meta).unwrap_or_default()
        };

        // Format the final message
        let message = format!("{}: {}", info.level, pretty_message);

        // Return a new LogInfo object with the pretty-printed message
        Some(LogInfo {
            level: info.level,
            message,
            meta: info.meta, // Original meta preserved
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use serde_json::json;

    #[test]
    fn test_pretty_print_formatter() {
        let formatter = pretty_print();

        let info = LogInfo::new("info", "User logged in")
            .add_meta("user_id", 12345)
            .add_meta("session_id", "abcde12345")
            .add_meta("extra_info", json!({"key": "value"}));

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);

        // Expected output:
        // info: {
        //   "user_id": 12345,
        //   "session_id": "abcde12345",
        //   "extra_info": {
        //     "key": "value"
        //   }
        // }
    }
}
