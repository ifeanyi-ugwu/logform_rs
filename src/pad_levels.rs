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
        &mut self,
        mut info: LogInfo,
        opts: &Option<HashMap<String, String>>,
    ) -> Option<LogInfo> {
        self.update_with_options(opts.clone());

        if let Some(padding) = self.paddings.get(&info.level) {
            info.message = format!("{}{}", padding, info.message); // Prepend padding to message
            return Some(info); // Return the transformed info with padding
        }

        // If no padding is applied, return the info unchanged (though this case shouldn't occur with default levels)
        Some(info)
    }

    pub fn update_with_options(&mut self, opts: FormatOptions) {
        if let Some(opts) = opts {
            if let Some(levels_str) = opts.get("levels") {
                if let Ok(levels) = serde_json::from_str::<HashMap<String, String>>(levels_str) {
                    let filler = opts.get("filler").map(|s| s.as_str()).unwrap_or(" ");
                    self.paddings = Self::padding_for_levels(&levels, filler);
                }
            }
        }
    }
}

pub fn pad_levels() -> Format {
    let padder = Padder::new(None);
    Format::new(move |info: LogInfo, options: FormatOptions| {
        let mut padder = padder.clone();
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
        let mut padder = Padder::new(Some(HashMap::from([(
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
        let mut padder = Padder::new(Some(HashMap::from([
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

    #[test]
    fn test_padlevels_function() {
        // Custom levels with varying lengths
        let levels = HashMap::from([
            ("info".to_string(), "info".to_string()),   // 4 characters
            ("error".to_string(), "error".to_string()), // 5 characters
            ("critical".to_string(), "critical".to_string()), // 8 characters
        ]);

        // Create the `padlevels` format with custom levels and filler
        let formatter = pad_levels()
            .with_option("levels", &serde_json::to_string(&levels).unwrap())
            .with_option("filler", "-");

        // Log message for "info" level
        let info = LogInfo::new("info", "Custom filler message");
        let result_info = formatter.transform(info, None).unwrap();

        // Log message for "error" level
        let error = LogInfo::new("error", "Error message");
        let result_error = formatter.transform(error, None).unwrap();

        // Log message for "critical" level
        let critical = LogInfo::new("critical", "Critical issue");
        let result_critical = formatter.transform(critical, None).unwrap();

        // Assert that padding is applied correctly
        assert_eq!(result_info.message, "-----Custom filler message"); // 8 - 4 + 1 = 5 dashes
        assert_eq!(result_error.message, "----Error message"); // 8 - 5 + 1 = 4 dashes
        assert_eq!(result_critical.message, "-Critical issue"); // No padding needed, already longest level
    }
}
