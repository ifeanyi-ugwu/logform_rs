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

/*
pub struct ColorizeOptions {
    pub all: bool,
    pub level: bool,
    pub message: bool,
}

pub struct Colorizer {
    colors: HashMap<String, String>,
    options: ColorizeOptions,
    color_map: HashMap<String, String>,
}

fn get_color_code_map() -> HashMap<String, String> {
    let mut color_map = HashMap::new();
    color_map.insert("black".to_string(), "\x1b[30m".to_string());
    color_map.insert("red".to_string(), "\x1b[31m".to_string());
    color_map.insert("green".to_string(), "\x1b[32m".to_string());
    color_map.insert("yellow".to_string(), "\x1b[33m".to_string());
    color_map.insert("blue".to_string(), "\x1b[34m".to_string());
    color_map.insert("magenta".to_string(), "\x1b[35m".to_string());
    color_map.insert("cyan".to_string(), "\x1b[36m".to_string());
    color_map.insert("white".to_string(), "\x1b[37m".to_string());
    color_map.insert("reset".to_string(), "\x1b[0m".to_string());
    color_map
}

impl Colorizer {
    pub fn new(colors: HashMap<String, String>, options: Option<ColorizeOptions>) -> Self {
        let color_map = get_color_code_map();
        let options = options.unwrap_or_else(|| ColorizeOptions {
            all: false,
            level: true,
            message: false,
        });

        Self {
            colors,
            options,
            color_map,
        }
    }

    fn colorize(&self, color_code: &str, text: &str) -> String {
        //format!("{}{}{}", color_code, text, "\x1b[0m")
        format!("{}{}{}", color_code, text, self.color_map["reset"])
    }
}

impl LogFormat for Colorizer {
    fn transform(&self, info: LogInfo) -> Option<LogInfo> {
        // Get the color name for the level
        let level_color_name = self
            .colors
            .get(&info.level)
            .unwrap_or(&self.color_map["reset"]);

        // Look up the color by name and then map it to the ANSI code
        let color_code = self
            .color_map
            .get(level_color_name)
            .unwrap_or(&self.color_map["reset"]);

        let mut new_info = info.clone();

        if self.options.all || (self.options.message && !info.message.is_empty()) {
            new_info.message = self.colorize(color_code, &info.message);
        }

        if self.options.level || self.options.all {
            new_info.level = self.colorize(color_code, &info.level);
        }

        Some(new_info)
    }
}
/*
pub fn colorize(color: &str) -> ColorizeFormat {
    ColorizeFormat::new(color)
}
*/
pub fn colorize(
    colors: HashMap<String, String>,
    options: Option<ColorizeOptions>,
) -> BoxedLogFormat {
    Box::new(Colorizer::new(colors, options))
}

pub struct ColorizerBuilder {
    colors: HashMap<String, String>,
    options: ColorizeOptions,
}

impl ColorizerBuilder {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
            options: ColorizeOptions {
                all: false,
                level: true,
                message: false,
            },
        }
    }

    pub fn add_color(mut self, level: &str, color: &str) -> Self {
        self.colors.insert(level.to_string(), color.to_string());
        self
    }

    pub fn set_all(mut self, all: bool) -> Self {
        self.options.all = all;
        self
    }

    pub fn set_level(mut self, level: bool) -> Self {
        self.options.level = level;
        self
    }

    pub fn set_message(mut self, message: bool) -> Self {
        self.options.message = message;
        self
    }

    pub fn build(self) -> BoxedLogFormat {
        Box::new(Colorizer::new(self.colors, Some(self.options)))
    }
}

pub fn colorize_builder() -> ColorizerBuilder {
    ColorizerBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{json, simple, timestamp};

    #[test]
    fn test_colorizer() {
        // Create a colorizer with options to colorize both level and message
        let colorizer_format = colorize_builder()
            .add_color("info", "green")
            .add_color("warn", "yellow")
            .add_color("error", "red")
            .set_all(true)
            .build();

        // Create a log info object
        let log_info = LogInfo::new("info", "This is an info message");

        // Apply the colorizer to the log info
        let log_info = colorizer_format.transform(log_info).unwrap();

        // Print the transformed (colored) log message to the console
        println!("{}: {}", log_info.level, log_info.message);
    }

    #[test]
    fn test_color_with_json_format() {
        // Create a LogInfo instance with some initial data
        let log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        // Apply JsonFormat
        let json_format = json();
        let log_info = json_format.transform(log_info).unwrap();

        // Define some colors for different log levels
        let colorizer_format = colorize_builder()
            .add_color("info", "green")
            .add_color("warn", "yellow")
            .add_color("error", "red")
            .set_level(true)
            .build();

        // Apply ColorizeFormat
        let log_info = colorizer_format.transform(log_info).unwrap();

        println!("{}", log_info.message);
    }

    #[test]
    fn test_color_with_simple_format() {
        // Create a LogInfo instance with some initial data
        let log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        let timestamp_format = timestamp(None);
        // Apply SimpleFormat
        let simple_format = simple();
        let log_info = timestamp_format.transform(log_info).unwrap();
        //simple_format.transform(&mut log_info);

        // Define some colors for different log levels
        let colorizer_format = colorize_builder()
            .add_color("info", "green")
            .add_color("warn", "yellow")
            .add_color("error", "red")
            .set_message(true)
            .build();

        // Apply ColorizeFormat
        let log_info = colorizer_format.transform(log_info).unwrap();
        let log_info = simple_format.transform(log_info).unwrap();

        println!("{}", log_info.message);
    }
}
*/
