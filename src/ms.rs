use crate::{Format, FormatOptions, LogInfo};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
    static ref PREV_TIME: Mutex<Instant> = Mutex::new(Instant::now());
}

pub fn ms() -> Format {
    Format::new(move |mut info: LogInfo, _options: FormatOptions| {
        let curr = Instant::now();
        let mut prev_time = PREV_TIME.lock().unwrap();
        let diff = curr.duration_since(*prev_time);
        *prev_time = curr;

        // Add the time difference in milliseconds to the `info` meta
        info.meta
            .insert("ms".to_string(), format!("+{}ms", diff.as_millis()).into());
        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_time_diff_format() {
        let formatter = ms();

        let info = LogInfo::new("info", "Test message").add_meta("key", "value");

        // First transformation
        let result = formatter.transform(info.clone(), None).unwrap();
        println!("Log info with time diff (first call): {:?}", result.meta);

        // Simulate a delay to test time difference
        sleep(Duration::from_millis(300));

        // Second transformation
        let result = formatter.transform(info.clone(), None).unwrap();
        println!("Log info with time diff (second call): {:?}", result.meta);

        // Verify that the `ms` field shows the difference
        assert!(result.meta.contains_key("ms"));

        // print the time differences
        println!("First MS field: {:?}", result.meta.get("ms"));
    }
}
