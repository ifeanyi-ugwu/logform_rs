use crate::LogInfo;
use std::sync::Arc;

use super::Format;

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
}

impl Format for Printf {
    type Input = LogInfo;

    fn transform(&self, mut info: LogInfo) -> Option<Self::Input> {
        info.message = (self.template)(&info);
        Some(info)
    }
}

pub fn printf<T>(template_fn: T) -> Printf
where
    T: Fn(&LogInfo) -> String + Send + Sync + 'static,
{
    Printf::new(Arc::new(template_fn))
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

        let info = LogInfo::new("info", "This is a message").with_meta("key", "value");

        let result = formatter.transform(info).unwrap();
        println!("{}", result.message); // Check the formatted output
    }
}
