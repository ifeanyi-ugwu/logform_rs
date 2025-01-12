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
            // Start with the original message
            color_entry
                .as_vec()
                .iter()
                .fold(message.to_string(), |colored_message, color| {
                    apply_color(&colored_message, color)
                })
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

fn apply_color(message: &str, color: &str) -> String {
    match color {
        // Foreground Colors
        "black" => message.black().to_string(),
        "red" => message.red().to_string(),
        "green" => message.green().to_string(),
        "yellow" => message.yellow().to_string(),
        "blue" => message.blue().to_string(),
        "magenta" => message.magenta().to_string(),
        "cyan" => message.cyan().to_string(),
        "white" => message.white().to_string(),
        // Bright Foreground Colors
        "bright_black" => message.bright_black().to_string(),
        "bright_red" => message.bright_red().to_string(),
        "bright_green" => message.bright_green().to_string(),
        "bright_yellow" => message.bright_yellow().to_string(),
        "bright_blue" => message.bright_blue().to_string(),
        "bright_magenta" => message.bright_magenta().to_string(),
        "bright_cyan" => message.bright_cyan().to_string(),
        "bright_white" => message.bright_white().to_string(),
        // Background Colors
        "on_black" => message.on_black().to_string(),
        "on_red" => message.on_red().to_string(),
        "on_green" => message.on_green().to_string(),
        "on_yellow" => message.on_yellow().to_string(),
        "on_blue" => message.on_blue().to_string(),
        "on_magenta" => message.on_magenta().to_string(),
        "on_cyan" => message.on_cyan().to_string(),
        "on_white" => message.on_white().to_string(),
        // Bright Background Colors
        "on_bright_black" => message.on_bright_black().to_string(),
        "on_bright_red" => message.on_bright_red().to_string(),
        "on_bright_green" => message.on_bright_green().to_string(),
        "on_bright_yellow" => message.on_bright_yellow().to_string(),
        "on_bright_blue" => message.on_bright_blue().to_string(),
        "on_bright_magenta" => message.on_bright_magenta().to_string(),
        "on_bright_cyan" => message.on_bright_cyan().to_string(),
        "on_bright_white" => message.on_bright_white().to_string(),
        // Styles
        "bold" => message.bold().to_string(),
        "underline" => message.underline().to_string(),
        "italic" => message.italic().to_string(),
        "dimmed" => message.dimmed().to_string(),
        "reversed" => message.reversed().to_string(),
        "blink" => message.blink().to_string(),
        "hidden" => message.hidden().to_string(),
        "strikethrough" => message.strikethrough().to_string(),
        // Default case
        _ => message.to_string(),
    }
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
