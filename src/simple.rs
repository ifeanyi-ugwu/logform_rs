use crate::{Format, FormatOptions, LogInfo};

pub fn simple() -> Format {
    Format::new(|info: LogInfo, _opts: FormatOptions| {
        // Get padding if present in meta, otherwise default to an empty string
        let padding = info
            .meta
            .get("padding")
            .and_then(|v| v.get(&info.level))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Start constructing the message with level, padding, and main message
        let mut message = format!("{}:{} {}", info.level, padding, info.message);

        // Clone the meta to filter out certain fields
        let mut rest = info.meta.clone();
        rest.remove("level");
        rest.remove("message");
        rest.remove("splat");
        rest.remove("padding"); // Remove the padding field

        // If there are remaining fields, stringify them and append to the message
        if !rest.is_empty() {
            let rest_string = serde_json::to_string(&rest).unwrap_or_default();
            message.push_str(&format!(" {}", rest_string));
        }

        // Return the new LogInfo with the constructed message
        Some(LogInfo {
            level: info.level,
            message,
            meta: info.meta,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_formatter() {
        let formatter = simple();

        let info = LogInfo::new("info", "User logged in")
            .add_meta("user_id", 12345)
            .add_meta("session_id", "abcde12345")
            .add_meta("padding", json!({"info": "    "}));

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message);

        // Expected output:
        // info:    User logged in {"user_id":12345,"session_id":"abcde12345"}
    }
}
