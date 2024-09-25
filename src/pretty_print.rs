use crate::{utils::format_json::format_json, Format, FormatOptions, LogInfo};
use serde_json::Value;

pub fn pretty_print() -> Format {
    Format::new(|info: LogInfo, opts: FormatOptions| {
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

        // Create a new JSON object with level and message
        let mut json_output = serde_json::Map::new();
        json_output.insert("level".to_string(), Value::String(info.level.to_string()));
        json_output.insert("message".to_string(), Value::String(info.message.clone()));

        // Add the rest of the meta data
        for (key, value) in meta {
            json_output.insert(key, value);
        }

        // Convert the JSON object to a Value
        let json_value = Value::Object(json_output);

        // Format and apply color formatting to the entire JSON structure
        let pretty_message = format_json(&json_value, colorize);

        // Return a new LogInfo object with the pretty-printed message
        Some(LogInfo {
            level: info.level,
            message: pretty_message,
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
            .add_meta("an array", json!(["abcde12345", true, 2])).add_meta("empty object", json!({})).add_meta("empty array", json!([]));

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);
    }
}
