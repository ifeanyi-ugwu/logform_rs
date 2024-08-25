use std::collections::HashMap;

#[derive(Debug)]
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, String>,
}

pub trait LogFormat {
    fn transform<'a>(&'a self, info: &'a mut LogInfo) -> Option<&'a LogInfo>;
}

pub struct Format<F>
where
    F: for<'a> Fn(&'a mut LogInfo, Option<&'a HashMap<String, String>>) -> Option<&'a LogInfo>
        + Clone,
{
    format_fn: F,
    options: Option<HashMap<String, String>>,
}

impl<F> LogFormat for Format<F>
where
    F: for<'a> Fn(&'a mut LogInfo, Option<&'a HashMap<String, String>>) -> Option<&'a LogInfo>
        + Clone,
{
    fn transform<'a>(&'a self, info: &'a mut LogInfo) -> Option<&'a LogInfo> {
        (self.format_fn)(info, self.options.as_ref())
    }
}

pub fn create_format<F>(format_fn: F) -> impl Fn(Option<HashMap<String, String>>) -> Format<F>
where
    F: for<'a> Fn(&'a mut LogInfo, Option<&'a HashMap<String, String>>) -> Option<&'a LogInfo>
        + Clone,
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
            |info: &mut LogInfo, opts: Option<&HashMap<String, String>>| {
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

        let mut info = LogInfo {
            level: "info".to_string(),
            message: "sorry for making you YELL in your head!".to_string(),
            meta: HashMap::new(),
        };

        scream.transform(&mut info);
        println!("{:?}", info);

        let mut whisper_opts = HashMap::new();
        whisper_opts.insert("whisper".to_string(), "true".to_string());
        let whisper = volume(Some(whisper_opts));

        let mut info2 = LogInfo {
            level: "info".to_string(),
            message: "WHY ARE THEY MAKING US YELL SO MUCH!".to_string(),
            meta: HashMap::new(),
        };

        whisper.transform(&mut info2);
        println!("{:?}", info2);
    }

    #[test]
    fn test_ignore_private() {
        let ignore_private = create_format(
            |info: &mut LogInfo, _opts: Option<&HashMap<String, String>>| {
                if let Some(private) = info.meta.get("private") {
                    if private == "true" {
                        return None;
                    }
                }
                Some(info)
            },
        );

        let format = ignore_private(None);

        let mut public_info = LogInfo {
            level: "error".to_string(),
            message: "Public error to share".to_string(),
            meta: HashMap::new(),
        };
        public_info
            .meta
            .insert("private".to_string(), "false".to_string());

        let result = format.transform(&mut public_info);
        println!("{:?}", result); // Should print Some(LogInfo { ... })

        let mut private_info = LogInfo {
            level: "error".to_string(),
            message: "This is super secret - hide it.".to_string(),
            meta: HashMap::new(),
        };
        private_info
            .meta
            .insert("private".to_string(), "true".to_string());

        let result = format.transform(&mut private_info);
        println!("{:?}", result); // Should print None
    }
}
