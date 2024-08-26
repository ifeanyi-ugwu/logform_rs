use crate::{create_format, format_json::format_json, Format, FormatOptions, LogInfo};
use serde_json::Value;

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

        // Convert meta to a serde_json::Value
        let meta_value: Value = serde_json::to_value(meta).unwrap_or(Value::Null);

        // Apply color formatting to each value in the meta
        let pretty_message = format_json(&meta_value, colorize);

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
    use serde_json::json;

    #[test]
    fn test_pretty_print_formatter() {
        let formatter = pretty_print().with_option("colorize", "true");

        let info = LogInfo::new("info", "User logged in")
            .add_meta("user_id", 12345)
            .add_meta("session_id", "abcde12345")
            .add_meta(
                "extra_info",
                json!({"null": null,"number": 1,"boolean": true,"inner_object":{"null": null,"number": 1,"boolean": true,}}),
            )
            .add_meta("an array", json!(["abcde12345", true, 2]));

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
