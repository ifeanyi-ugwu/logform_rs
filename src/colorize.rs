use crate::{config, Format, FormatOptions, LogInfo};
use colored::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum MixedColorType {
    Single(String),
    Multiple(Vec<String>),
}

impl MixedColorType {
    // Helper method to get all colors as a Vec<String>
    fn as_vec(&self) -> Vec<String> {
        match self {
            MixedColorType::Single(color) => vec![color.clone()],
            MixedColorType::Multiple(colors) => colors.clone(),
        }
    }
}

impl From<String> for MixedColorType {
    fn from(value: String) -> Self {
        MixedColorType::Single(value)
    }
}

impl From<Vec<String>> for MixedColorType {
    fn from(values: Vec<String>) -> Self {
        MixedColorType::Multiple(values)
    }
}

#[derive(Clone)]
pub struct Colorizer {
    all_colors: HashMap<String, MixedColorType>,
    options: HashMap<String, String>,
}

impl Colorizer {
    pub fn new(opts: Option<HashMap<String, String>>) -> Self {
        let all_colors = config::rust::colors()
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect();

        let options = opts.unwrap_or_default();

        let mut colorizer = Colorizer {
            all_colors,
            options,
        };

        if let Some(colors) = colorizer.options.get("colors") {
            // Parse the colors string and add to all_colors
            let color_map: HashMap<String, serde_json::Value> =
                serde_json::from_str(colors).unwrap_or_default();
            colorizer.add_colors(color_map);
        }

        colorizer
    }

    pub fn add_colors(&mut self, colors: HashMap<String, serde_json::Value>) {
        for (level, color_val) in colors {
            let color_entry: MixedColorType = match color_val {
                serde_json::Value::String(color_str) => color_str.into(),
                serde_json::Value::Array(color_arr) => color_arr
                    .into_iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
                    .into(),
                // _ => continue, // Skip unexpected formats
                _ => {
                    eprintln!("Unexpected format for color value: {:?}", color_val);
                    continue;
                }
            };
            self.all_colors.insert(level, color_entry);
        }
    }

    pub fn colorize(&self, level: &str, message: &str) -> String {
        if let Some(color_entry) = self.all_colors.get(level) {
            let mut colored_message = message.to_string();
            for color in color_entry.as_vec() {
                colored_message = match color.as_str() {
                    // Foreground Colors
                    "black" => colored_message.black().to_string(),
                    "red" => colored_message.red().to_string(),
                    "green" => colored_message.green().to_string(),
                    "yellow" => colored_message.yellow().to_string(),
                    "blue" => colored_message.blue().to_string(),
                    "magenta" => colored_message.magenta().to_string(),
                    "cyan" => colored_message.cyan().to_string(),
                    "white" => colored_message.white().to_string(),
                    // Bright Foreground Colors
                    "bright_black" => colored_message.bright_black().to_string(),
                    "bright_red" => colored_message.bright_red().to_string(),
                    "bright_green" => colored_message.bright_green().to_string(),
                    "bright_yellow" => colored_message.bright_yellow().to_string(),
                    "bright_blue" => colored_message.bright_blue().to_string(),
                    "bright_magenta" => colored_message.bright_magenta().to_string(),
                    "bright_cyan" => colored_message.bright_cyan().to_string(),
                    "bright_white" => colored_message.bright_white().to_string(),
                    // Background Colors
                    "on_black" => colored_message.on_black().to_string(),
                    "on_red" => colored_message.on_red().to_string(),
                    "on_green" => colored_message.on_green().to_string(),
                    "on_yellow" => colored_message.on_yellow().to_string(),
                    "on_blue" => colored_message.on_blue().to_string(),
                    "on_magenta" => colored_message.on_magenta().to_string(),
                    "on_cyan" => colored_message.on_cyan().to_string(),
                    "on_white" => colored_message.on_white().to_string(),
                    // Bright Background Colors
                    "on_bright_black" => colored_message.on_bright_black().to_string(),
                    "on_bright_red" => colored_message.on_bright_red().to_string(),
                    "on_bright_green" => colored_message.on_bright_green().to_string(),
                    "on_bright_yellow" => colored_message.on_bright_yellow().to_string(),
                    "on_bright_blue" => colored_message.on_bright_blue().to_string(),
                    "on_bright_magenta" => colored_message.on_bright_magenta().to_string(),
                    "on_bright_cyan" => colored_message.on_bright_cyan().to_string(),
                    "on_bright_white" => colored_message.on_bright_white().to_string(),
                    // Styles
                    "bold" => colored_message.bold().to_string(),
                    "underline" => colored_message.underline().to_string(),
                    "italic" => colored_message.italic().to_string(),
                    "dimmed" => colored_message.dimmed().to_string(),
                    "reversed" => colored_message.reversed().to_string(),
                    "blink" => colored_message.blink().to_string(),
                    "hidden" => colored_message.hidden().to_string(),
                    "strikethrough" => colored_message.strikethrough().to_string(),
                    // Default case
                    _ => colored_message,
                };
            }
            colored_message
        } else {
            message.to_string()
        }
    }

    pub fn transform(
        &mut self,
        mut info: LogInfo,
        opts: Option<HashMap<String, String>>,
    ) -> Option<LogInfo> {
        if let Some(incoming_opts) = opts {
            self.merge_options(incoming_opts);
        }

        let all = self
            .options
            .get("all")
            .map(|v| v == "true")
            .unwrap_or(false);
        let level = self
            .options
            .get("level")
            .map(|v| v == "true")
            .unwrap_or(false);
        let message = self
            .options
            .get("message")
            .map(|v| v == "true")
            .unwrap_or(false);

        // Store original level for color lookup
        let original_level = info.level.clone();

        if all || level || !message {
            info.level = self.colorize(&original_level, &info.level);
        }

        if all || message {
            info.message = self.colorize(&original_level, &info.message);
        }

        Some(info)
    }

    pub fn merge_options(&mut self, opts: HashMap<String, String>) {
        self.options.extend(opts);
        if let Some(colors) = self.options.get("colors") {
            let color_map: HashMap<String, serde_json::Value> =
                serde_json::from_str(colors).unwrap_or_default();
            self.add_colors(color_map);
        }
    }
}

fn default_colors() -> HashMap<String, Vec<String>> {
    let mut defaults = HashMap::new();
    defaults.insert("error".to_string(), vec!["red".to_string()]);
    defaults.insert("warn".to_string(), vec!["yellow".to_string()]);
    defaults.insert("info".to_string(), vec!["green".to_string()]);
    defaults.insert("debug".to_string(), vec!["blue".to_string()]);
    defaults.insert("trace".to_string(), vec!["magenta".to_string()]);
    defaults
}

pub fn colorize() -> Format {
    let colorizer = Colorizer::new(None);
    Format::new(move |info: LogInfo, options: FormatOptions| {
        let mut colorizer = colorizer.clone();
        colorizer.transform(info, options)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::control::set_override;
    use serde_json::json;

    #[test]
    fn test_colorize_formatter() {
        // Force colored output even if not in a TTY environment
        set_override(true);

        let formatter = colorize()
            .with_option(
                "colors",
                &json!({"info": "blue", "error": ["red", "bold"]}).to_string(),
            )
            .with_option("all", "true");

        let info = LogInfo::new("info", "This is an info message").with_meta("key", "value");

        let result = formatter.transform(info, None).unwrap();
        println!("Colorized info: {} - {}", result.level, result.message);

        let error_info = LogInfo::new("error", "This is an error message");

        let result_error = formatter.transform(error_info, None).unwrap();
        println!(
            "Colorized error: {} - {}",
            result_error.level, result_error.message
        );
    }
}
