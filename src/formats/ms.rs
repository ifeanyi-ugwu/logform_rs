use crate::LogInfo;
use std::sync::Mutex;
use std::time::Instant;

use super::Format;

pub struct MsFormat {
    prev_time: Mutex<Instant>,
}

impl MsFormat {
    pub fn new() -> Self {
        MsFormat {
            prev_time: Mutex::new(Instant::now()),
        }
    }
}

impl Format for MsFormat {
    type Input = LogInfo;

    fn transform(&self, mut input: LogInfo) -> Option<Self::Input> {
        let curr = Instant::now();
        let mut prev_time = self.prev_time.lock().ok()?;
        let diff = curr.duration_since(*prev_time);
        *prev_time = curr;

        // Add the time difference in milliseconds to the `info` meta
        input
            .meta
            .insert("ms".to_string(), format!("+{}ms", diff.as_millis()).into());

        Some(input)
    }
}

pub fn ms() -> MsFormat {
    MsFormat::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_ms_format() {
        let formatter = ms();
        let info = LogInfo::new("info", "Test message");

        // First transformation (initial reference point)
        formatter.transform(info.clone()).unwrap();

        // Simulate a delay
        sleep(Duration::from_millis(300));

        // Second transformation
        let result2 = formatter.transform(info.clone()).unwrap();
        let ms2 = result2.meta.get("ms").unwrap().as_str().unwrap();
        let ms2_value: u64 = ms2
            .trim_start_matches('+')
            .trim_end_matches("ms")
            .parse()
            .unwrap();

        assert!(
            (250..350).contains(&ms2_value),
            "Expected ~300ms, but got {}ms",
            ms2_value
        );
    }
}
