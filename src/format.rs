use crate::LogInfo;
use std::fmt;
use std::{collections::HashMap, sync::Arc};

pub type FormatOptions = Option<HashMap<String, String>>;

// a cloneable trait object
type BoxedLogFormatFn = Arc<dyn Fn(LogInfo, FormatOptions) -> Option<LogInfo> + Send + Sync>;

pub struct Format {
    pub format_fn: BoxedLogFormatFn,
    pub options: FormatOptions,
}

impl Format {
    pub fn new<F>(format_fn: F) -> Self
    where
        F: Fn(LogInfo, FormatOptions) -> Option<LogInfo> + Send + Sync + 'static,
    {
        Format {
            format_fn: Arc::new(format_fn),
            options: None,
        }
    }

    pub fn transform(&self, info: LogInfo, opts: FormatOptions) -> Option<LogInfo> {
        let merged_opts = self.merge_options(opts);
        (self.format_fn)(info, merged_opts)
    }

    pub fn with_option<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.options
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    fn merge_options(&self, opts: FormatOptions) -> FormatOptions {
        match (&self.options, opts) {
            (None, None) => None,
            (Some(existing), None) => Some(existing.clone()),
            (None, Some(new_opts)) => Some(new_opts),
            (Some(existing), Some(mut new_opts)) => {
                let mut final_opts = existing.clone();
                final_opts.extend(new_opts.drain());
                Some(final_opts)
            }
        }
    }
}

impl Clone for Format {
    fn clone(&self) -> Self {
        Format {
            format_fn: Arc::clone(&self.format_fn),
            options: self.options.clone(),
        }
    }
}

impl fmt::Debug for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // I cannot directly debug the closure, so I provided a placeholder
        f.debug_struct("Format")
            .field("format_fn", &"Function pointer") // Placeholder for the function pointer
            .field("options", &self.options)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_custom_format() {
        let volume = Format::new(|mut info: LogInfo, opts: FormatOptions| {
            if let Some(opts) = opts {
                if opts.get("yell").is_some() {
                    info.message = info.message.to_uppercase();
                } else if opts.get("whisper").is_some() {
                    info.message = info.message.to_lowercase();
                }
            }
            Some(info)
        });

        let mut scream_opts = HashMap::new();
        scream_opts.insert("yell".to_string(), "true".to_string());
        let scream = volume.clone();

        let info = LogInfo::new("info", "sorry for making you YELL in your head!");

        let result = scream.transform(info, Some(scream_opts)).unwrap();
        println!("{}", result.message);

        let mut whisper_opts = HashMap::new();
        whisper_opts.insert("whisper".to_string(), "true".to_string());
        let whisper = volume;

        let info2 = LogInfo::new("info", "WHY ARE THEY MAKING US YELL SO MUCH!");

        let result2 = whisper.transform(info2, Some(whisper_opts)).unwrap();
        println!("{}", result2.message);
    }

    #[test]
    fn test_ignore_private() {
        let ignore_private = Format::new(|info: LogInfo, _opts: FormatOptions| {
            if let Some(private) = info.meta.get("private") {
                if private == "true" {
                    return None;
                }
            }
            Some(info)
        });

        let format = ignore_private;

        let public_info =
            LogInfo::new("error", "Public error to share").add_meta("private", "false");

        let result = format.transform(public_info, None).unwrap();
        println!("{}", result.message);

        let private_info =
            LogInfo::new("error", "This is super secret - hide it.").add_meta("private", "true");

        let result = format.transform(private_info, None);
        println!("{:?}", result);
    }
}
