use logform::{colorize, combine, json, printf, simple, timestamp, LogFormat, LogInfo};

#[test]
pub fn initialize_and_test_formats() {
    let log_info = LogInfo::new("info", "This is a test message");

    let format = combine(vec![
        timestamp(),
        colorize(None)
            .with_option(
                "colors",
                &serde_json::json!({"info": ["blue"], "error": ["red", "bold"]}).to_string(),
            )
            .with_option("all", "true"),
        printf(|info| {
            let timestamp = info
                .meta
                .get("timestamp")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            //.map(|v| v.to_string())
            //.unwrap_or_else(|| "".to_string());

            format!("{} - {}: {}", timestamp, info.level, info.message)
        }),
    ]);

    let log_info = format.transform(log_info, None).unwrap();
    println!("{}", log_info.message);
}

#[test]
fn test_json() {
    let log_info = LogInfo::new("info", "This is a test message");

    // Apply the simple format
    let simple_format = simple();
    let log_info = simple_format.transform(log_info, None).unwrap();
    println!("Simple format: {}", log_info.message);

    // Reset log_info for JSON format
    let log_info = LogInfo::new("info", "This is a test message");

    // Apply the JSON format
    let json_format = json();
    let log_info = json_format.transform(log_info, None).unwrap();
    println!("JSON format: {}", log_info.message);
}
