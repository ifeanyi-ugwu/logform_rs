use std::collections::HashMap;

use crate::{Format, FormatOptions, LogInfo};

#[derive(Clone)]
pub struct Padder {
    paddings: HashMap<String, String>,
    _options: HashMap<String, String>,
}

impl Padder {
    // Create a new Padder
    pub fn new(opts: Option<HashMap<String, String>>) -> Self {
        let levels = opts
            .as_ref()
            .and_then(|o| o.get("levels"))
            .map(|s| serde_json::from_str::<HashMap<String, String>>(s).unwrap_or_default())
            .unwrap_or_else(|| {
                HashMap::from([
                    ("error".to_string(), "error".to_string()),
                    ("warn".to_string(), "warn".to_string()),
                    ("info".to_string(), "info".to_string()),
                    ("debug".to_string(), "debug".to_string()),
                    ("trace".to_string(), "trace".to_string()),
                ])
            }); // Default levels

        let binding = " ".to_string();
        let filler = opts
            .as_ref()
            .and_then(|o| o.get("filler"))
            .unwrap_or(&binding); // Default filler is space

        let paddings = Self::padding_for_levels(&levels, filler);

        Padder {
            paddings,
            _options: opts.unwrap_or_default(),
        }
    }

    // Get the maximum length of the log levels
    fn get_longest_level(levels: &HashMap<String, String>) -> usize {
        levels.keys().map(|level| level.len()).max().unwrap_or(0)
    }

    // Get the padding required for each level
    fn padding_for_levels(
        levels: &HashMap<String, String>,
        filler: &str,
    ) -> HashMap<String, String> {
        let max_length = Self::get_longest_level(levels);
        levels
            .iter()
            .map(|(level, _)| {
                let padding = Self::padding_for_level(level, filler, max_length);
                (level.clone(), padding)
            })
            .collect()
    }

    // Get the padding for a single level
    fn padding_for_level(level: &str, filler: &str, max_length: usize) -> String {
        let target_len = max_length + 1 - level.len();
        let rep = target_len / filler.len();
        let padding = format!("{}{}", filler, filler.repeat(rep));
        padding.chars().take(target_len).collect()
    }

    // Transform the log info, adding padding
    pub fn transform(
        &self,
        mut info: LogInfo,
        _opts: &Option<HashMap<String, String>>,
    ) -> Option<LogInfo> {
        if let Some(padding) = self.paddings.get(&info.level) {
            info.message = format!("{}{}", padding, info.message); // Prepend padding to message
            return Some(info); // Return the transformed info with padding
        }

        // If no padding is applied, return the info unchanged (though this case shouldn't occur with default levels)
        Some(info)
    }
}

pub fn padlevels() -> Format {
    let padder = Padder::new(None);
    Format::new(move |info: LogInfo, options: FormatOptions| {
        //let padder = padder.clone();
        padder.transform(info, &options)
    })
}

#[cfg(test)]
mod padder_tests {
    use super::*;
    use crate::LogInfo;
    use std::collections::HashMap;

    #[test]
    fn test_padder_with_padding() {
        // Log levels with different lengths
        let levels = HashMap::from([
            ("info".to_string(), "info".to_string()),
            ("error".to_string(), "error".to_string()),
        ]);
        let padder = Padder::new(Some(HashMap::from([(
            "levels".to_string(),
            serde_json::to_string(&levels).unwrap(),
        )])));

        let log_info = LogInfo::new("error", "Test message");
        let transformed = padder.transform(log_info, &None);

        // The longest level is "error" (5 chars), so padding should be 1 space
        assert_eq!(transformed.unwrap().message, " Test message");
    }

    #[test]
    fn test_padder_with_custom_filler() {
        // Test padding with a custom filler character
        let levels = HashMap::from([
            ("info".to_string(), "info".to_string()),   // 4 characters
            ("debug".to_string(), "debug".to_string()), // 5 characters
            ("critical".to_string(), "critical".to_string()), // 8 characters
        ]);
        let padder = Padder::new(Some(HashMap::from([
            (
                "levels".to_string(),
                serde_json::to_string(&levels).unwrap(),
            ),
            ("filler".to_string(), "#".to_string()),
        ])));

        let log_info = LogInfo::new("debug", "Test message");
        let transformed = padder.transform(log_info, &None);

        let log_info = LogInfo::new("info", "Test message");
        let transformed_2 = padder.transform(log_info, &None);

        // The longest level is "critical" (8 chars), so padding for "debug" should be 4 "#" (i.e max_length + 1 - current_level_length)
        assert_eq!(transformed.unwrap().message, "####Test message");
        // The longest level is "critical" (8 chars), so padding for "info" should be 5 "#" (i.e max_length + 1 - current_level_length)
        assert_eq!(transformed_2.unwrap().message, "#####Test message");
    }
}
