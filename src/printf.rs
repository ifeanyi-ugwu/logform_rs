use crate::{create_format, Format, FormatOptions, LogInfo};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Printf {
    template: Arc<dyn Fn(&LogInfo) -> String + Send + Sync>,
}

impl Printf {
    pub fn new(template_fn: Arc<dyn Fn(&LogInfo) -> String + Send + Sync>) -> Self {
        Printf {
            template: template_fn,
        }
    }

    pub fn transform(
        &self,
        mut info: LogInfo,
        _opts: Option<HashMap<String, String>>,
    ) -> Option<LogInfo> {
        info.message = (self.template)(&info);
        Some(info)
    }
}

pub fn printf<T>(template_fn: T) -> Format
where
    T: Fn(&LogInfo) -> String + Send + Sync + 'static,
{
    let printf_formatter = Printf::new(Arc::new(template_fn));
    create_format(move |info: LogInfo, options: FormatOptions| {
        printf_formatter.transform(info, options.map(|o| o.clone()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogFormat;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_printf_formatter() {
        let formatter = printf(|info: &LogInfo| {
            format!(
                "{} - {}: {}",
                info.level,
                info.message,
                serde_json::to_string(&info.meta).unwrap_or_default()
            )
        });

        let mut meta = HashMap::new();
        meta.insert("key".to_string(), json!("value"));

        let info = LogInfo {
            level: "info".to_string(),
            message: "This is a message".to_string(),
            meta,
        };

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message); // Check the formatted output
    }
}

/*
pub struct PrintfFormat<F: Fn(&LogInfo) -> String + Send + Sync> {
    formatter: F,
}

impl<F> PrintfFormat<F>
where
    F: Fn(&LogInfo) -> String + Send + Sync,
{
    pub fn new(formatter: F) -> Self {
        Self { formatter }
    }
}

impl<F> LogFormat for PrintfFormat<F>
where
    F: Fn(&LogInfo) -> String + Send + Sync,
{
    fn transform(&self, info: LogInfo, opts: Option<&HashMap<String, String>>) -> Option<LogInfo> {
        let formatted_message = (self.formatter)(&info);
        Some(LogInfo {
            level: info.level,
            message: formatted_message,
            meta: info.meta,
        })
    }
}
/*
pub fn printf<F>(formatter: F) -> PrintfFormat<F>
where
    F: Fn(&LogInfo) -> String + 'static,
{
    PrintfFormat::new(formatter)
}
*/

pub fn printf<F>(formatter: F) -> BoxedLogFormat
where
    F: Fn(&LogInfo) -> String + Send + Sync + 'static,
{
    Box::new(PrintfFormat::new(formatter))
}
*/
