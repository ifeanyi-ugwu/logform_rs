use crate::{create_format, Format, FormatOptions, LogInfo};
use colored::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Colorizer {
    all_colors: HashMap<String, Vec<String>>,
    options: HashMap<String, String>,
}

impl Colorizer {
    pub fn new(opts: Option<HashMap<String, String>>) -> Self {
        let all_colors = HashMap::new();
        let options = opts.unwrap_or_default();

        let mut colorizer = Colorizer {
            all_colors,
            options,
        };

        if let Some(colors) = colorizer.options.get("colors") {
            // Parse the colors string and add to all_colors
            let color_map: HashMap<String, Vec<String>> =
                serde_json::from_str(colors).unwrap_or_default();
            colorizer.add_colors(color_map);
        }

        colorizer
    }

    pub fn add_colors(&mut self, colors: HashMap<String, Vec<String>>) {
        self.all_colors.extend(colors);
    }

    pub fn colorize(&self, level: &str, message: &str) -> String {
        if let Some(color_list) = self.all_colors.get(level) {
            let mut colored_message = message.to_string();
            for color in color_list {
                colored_message = match color.as_str() {
                    "red" => colored_message.red().to_string(),
                    "green" => colored_message.green().to_string(),
                    "yellow" => colored_message.yellow().to_string(),
                    "blue" => colored_message.blue().to_string(),
                    "magenta" => colored_message.magenta().to_string(),
                    "cyan" => colored_message.cyan().to_string(),
                    "white" => colored_message.white().to_string(),
                    "bold" => colored_message.bold().to_string(),
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

        if self.options.get("all").is_some() {
            info.message = self.colorize(&info.level, &info.message);
            info.level = self.colorize(&info.level, &info.level);
        } else {
            if self.options.get("level").is_some() {
                info.level = self.colorize(&info.level, &info.level);
            }
            if self.options.get("message").is_some() {
                info.message = self.colorize(&info.level, &info.message);
            }
        }

        Some(info)
    }

    pub fn merge_options(&mut self, opts: HashMap<String, String>) {
        self.options.extend(opts);
        if let Some(colors) = self.options.get("colors") {
            let color_map: HashMap<String, Vec<String>> =
                serde_json::from_str(colors).unwrap_or_default();
            self.add_colors(color_map);
        }
    }
}

pub fn colorize(opts: Option<HashMap<String, String>>) -> Format {
    let colorizer = Colorizer::new(opts.clone());
    create_format(move |info: LogInfo, options: FormatOptions| {
        let mut colorizer = colorizer.clone();
        colorizer.transform(info, options)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use colored::control::set_override;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_colorize_formatter() {
        // Force colored output even if not in a TTY environment
        set_override(true);

        let mut opts = HashMap::new();
        opts.insert("all".to_string(), "true".to_string());
        opts.insert(
            "colors".to_string(),
            json!({
                "info": ["blue"],
                "error": ["red", "bold",]
            })
            .to_string(),
        );

        let formatter = colorize(None)
            .with_option(
                "colors",
                &json!({"info": ["blue"], "error": ["red", "bold"]}).to_string(),
            )
            .with_option("all", "true");

        let info = LogInfo::new("info", "This is an info message").add_meta("key", "value");

        let result = formatter.transform(info, Some(opts.clone())).unwrap();
        println!("Colorized info: {} - {}", result.level, result.message);

        let error_info = LogInfo::new("error", "This is an error message");

        let result_error = formatter.transform(error_info, Some(opts)).unwrap();
        println!(
            "Colorized error: {} - {}",
            result_error.level, result_error.message
        );
    }
}
