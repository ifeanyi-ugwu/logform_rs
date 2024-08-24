use crate::{BoxedLogFormat, LogFormat, LogInfo};
use std::collections::HashMap;

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
        format!("{}{}{}", color_code, text, "\x1b[0m")
    }
}

impl LogFormat for Colorizer {
    fn transform(&self, info: &mut LogInfo) {
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

        if self.options.all || (self.options.message && !info.message.is_empty()) {
            info.message = self.colorize(color_code, &info.message);
        }

        if self.options.level || self.options.all {
            info.level = self.colorize(color_code, &info.level);
        }
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
        let mut log_info = LogInfo::new("info", "This is an info message");

        // Apply the colorizer to the log info
        colorizer_format.transform(&mut log_info);

        // Print the transformed (colored) log message to the console
        println!("{}: {}", log_info.level, log_info.message);
    }

    #[test]
    fn test_color_with_json_format() {
        // Create a LogInfo instance with some initial data
        let mut log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        // Apply JsonFormat
        let json_format = json();
        json_format.transform(&mut log_info);

        // Define some colors for different log levels
        let colorizer_format = colorize_builder()
            .add_color("info", "green")
            .add_color("warn", "yellow")
            .add_color("error", "red")
            .set_level(true)
            .build();

        // Apply ColorizeFormat
        colorizer_format.transform(&mut log_info);

        println!("{}", log_info.message);
    }

    #[test]
    fn test_color_with_simple_format() {
        // Create a LogInfo instance with some initial data
        let mut log_info = LogInfo {
            level: "info".to_string(),
            message: "Test message".to_string(),
            meta: HashMap::new(),
        };

        let timestamp_format = timestamp(None);
        // Apply SimpleFormat
        let simple_format = simple();
        timestamp_format.transform(&mut log_info);
        //simple_format.transform(&mut log_info);

        // Define some colors for different log levels
        let colorizer_format = colorize_builder()
            .add_color("info", "green")
            .add_color("warn", "yellow")
            .add_color("error", "red")
            .set_message(true)
            .build();

        // Apply ColorizeFormat
        colorizer_format.transform(&mut log_info);
        simple_format.transform(&mut log_info);

        println!("{}", log_info.message);
    }
}
