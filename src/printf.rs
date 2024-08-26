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

        let info = LogInfo::new("info", "This is a message").add_meta("key", "value");

        let result = formatter.transform(info, None).unwrap();
        println!("{}", result.message); // Check the formatted output
    }
}
