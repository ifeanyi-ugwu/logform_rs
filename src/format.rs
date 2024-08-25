use crate::{LogFormat, LogInfo};
use std::collections::HashMap;

pub struct Format<F>
where
    F: Fn(LogInfo, Option<&HashMap<String, String>>) -> Option<LogInfo> + Clone,
{
    format_fn: F,
    options: Option<HashMap<String, String>>,
}

impl<F> LogFormat for Format<F>
where
    F: Fn(LogInfo, Option<&HashMap<String, String>>) -> Option<LogInfo> + Clone,
{
    fn transform(&self, info: LogInfo) -> Option<LogInfo> {
        (self.format_fn)(info, self.options.as_ref())
    }
}

pub fn create_format<F>(format_fn: F) -> impl Fn(Option<HashMap<String, String>>) -> Format<F>
where
    F: Fn(LogInfo, Option<&HashMap<String, String>>) -> Option<LogInfo> + Clone,
{
    move |opts: Option<HashMap<String, String>>| Format {
        format_fn: format_fn.clone(),
        options: opts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_custom_format() {
        let volume = create_format(
            |mut info: LogInfo, opts: Option<&HashMap<String, String>>| {
                if let Some(opts) = opts {
                    if opts.get("yell").is_some() {
                        info.message = info.message.to_uppercase();
                    } else if opts.get("whisper").is_some() {
                        info.message = info.message.to_lowercase();
                    }
                }
                Some(info)
            },
        );

        let mut scream_opts = HashMap::new();
        scream_opts.insert("yell".to_string(), "true".to_string());
        let scream = volume(Some(scream_opts));

        let info = LogInfo {
            level: "info".to_string(),
            message: "sorry for making you YELL in your head!".to_string(),
            meta: HashMap::new(),
        };

        let result = scream.transform(info).unwrap();
        println!("{}", result.message);

        let mut whisper_opts = HashMap::new();
        whisper_opts.insert("whisper".to_string(), "true".to_string());
        let whisper = volume(Some(whisper_opts));

        let info2 = LogInfo {
            level: "info".to_string(),
            message: "WHY ARE THEY MAKING US YELL SO MUCH!".to_string(),
            meta: HashMap::new(),
        };

        let result2 = whisper.transform(info2).unwrap();
        println!("{}", result2.message);
    }

    #[test]
    fn test_ignore_private() {
        let ignore_private =
            create_format(|info: LogInfo, _opts: Option<&HashMap<String, String>>| {
                if let Some(private) = info.meta.get("private") {
                    if private == "true" {
                        return None;
                    }
                }
                Some(info)
            });

        let format = ignore_private(None);

        let mut public_info = LogInfo::new("error", "Public error to share");

        public_info
            .meta
            .insert("private".to_string(), serde_json::json!("false"));

        let result = format.transform(public_info).unwrap();
        println!("{:?}", result.message);

        let mut private_info = LogInfo {
            level: "error".to_string(),
            message: "This is super secret - hide it.".to_string(),
            meta: HashMap::new(),
        };
        private_info
            .meta
            .insert("private".to_string(), serde_json::json!("true"));

        let result = format.transform(private_info);
        println!("{:?}", result);
    }
}
