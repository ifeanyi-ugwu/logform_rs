use logform::{colorize, combine, json, printf, simple, timestamp, LogInfo};

#[test]
pub fn initialize_and_test_formats() {
    let mut info = LogInfo::new("info", "This is a test message");

    let format = combine(vec![
        timestamp(None),
        colorize("31"),
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

    format.transform(&mut info);
    println!("{}", info.message);
}

#[test]
fn test_json() {
    let mut log_info = LogInfo::new("info", "This is a test message");

    // Apply the simple format
    let simple_format = simple();
    simple_format.transform(&mut log_info);
    println!("Simple format: {}", log_info.message);

    // Reset log_info for JSON format
    log_info.message = "This is a test message".to_string();

    // Apply the JSON format
    let json_format = json();
    json_format.transform(&mut log_info);
    println!("JSON format: {}", log_info.message);
}
