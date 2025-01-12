use crate::{colorize::Colorizer, config, pad_levels::Padder, Format, FormatOptions, LogInfo};
use std::collections::HashMap;

#[derive(Clone)]
struct CliFormat {
    colorizer: Colorizer,
    padder: Padder,
    _options: HashMap<String, String>,
}

impl CliFormat {
    fn new(opts: Option<HashMap<String, String>>) -> Self {
        // Set default levels if `opts` doesn't have it
        let mut opts = opts.unwrap_or_default();
        if !opts.contains_key("levels") {
            opts.insert(
                "levels".to_string(),
                serde_json::to_string(&default_levels()).unwrap(),
            );
        }

        let colorizer = Colorizer::new(Some(opts.clone()));
        let padder = Padder::new(Some(opts.clone()));
        CliFormat {
            colorizer,
            padder,
            _options: opts,
        }
    }

    fn transform(
        &mut self,
        info: LogInfo,
        opts: &Option<HashMap<String, String>>,
    ) -> Option<LogInfo> {
        self.padder
            .transform(info.clone(), opts)
            .and_then(|info| self.colorizer.transform(info, opts.clone()))
            .map(|mut info| {
                info.message = format!("{}:{}", info.level, info.message);
                info
            })
    }
}

fn default_levels() -> HashMap<String, usize> {
    let levels = HashMap::from([
        ("error".to_string(), 0),
        ("warn".to_string(), 1),
        ("help".to_string(), 2),
        ("data".to_string(), 3),
        ("info".to_string(), 4),
        ("debug".to_string(), 5),
        ("prompt".to_string(), 6),
        ("verbose".to_string(), 7),
        ("input".to_string(), 8),
        ("silly".to_string(), 9),
    ]);
    levels
}

pub fn cli() -> Format {
    let cli_format = CliFormat::new(None);
    Format::new(move |info: LogInfo, options: FormatOptions| {
        let mut cli_format = cli_format.clone();
        cli_format.transform(info, &options)
    })
}

#[cfg(test)]
mod cli_format_tests {
    use super::*;
    use crate::LogInfo;
    use colored::control::set_override;
    use std::collections::HashMap;

    #[test]
    fn test_cli_format_with_custom_filler_and_color() {
        // Force colored output even if not in a TTY environment
        set_override(true);

        let levels = HashMap::from([
            ("info".to_string(), "info".to_string()),
            ("error".to_string(), "error".to_string()),
        ]);

        let mut cli_format = CliFormat::new(Some(HashMap::from([
            (
                "levels".to_string(),
                serde_json::to_string(&levels).unwrap(),
            ),
            ("filler".to_string(), "#".to_string()),
        ])));

        // Test error level
        let log_info = LogInfo::new("error", "Test message");
        let transformed = cli_format.transform(log_info, &None).unwrap();

        assert_eq!(
            transformed.message,
            format!("\x1b[31merror\x1b[0m:#Test message")
        );

        // Test info level
        let log_info = LogInfo::new("info", "Another test message");
        let transformed = cli_format.transform(log_info, &None).unwrap();

        assert_eq!(
            transformed.message,
            format!("\x1b[32minfo\x1b[0m:##Another test message")
        );
    }

    #[test]
    fn test_cli_function() {
        // Force colored output even if not in a TTY environment
        set_override(true);

        let levels = HashMap::from([
            ("info".to_string(), "info".to_string()),
            ("error".to_string(), "error".to_string()),
        ]);

        let formatter = cli()
            .with_option("levels", &serde_json::to_string(&levels).unwrap())
            .with_option("filler", "*")
            .with_option("all", "true");

        // Test "info" level
        let info = LogInfo::new("info", "Custom filler message");
        let transformed = formatter.transform(info, None).unwrap();

        assert_eq!(
            transformed.message,
            format!("\x1b[32minfo\x1b[0m:\x1b[32m**Custom filler message\x1b[0m")
        );

        // Test "error" level
        let error = LogInfo::new("error", "Custom error message");
        let transformed = formatter.transform(error, None).unwrap();

        assert_eq!(
            transformed.message,
            format!("\x1b[31merror\x1b[0m:\x1b[31m*Custom error message\x1b[0m")
        );
    }
}
