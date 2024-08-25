use crate::log_alt::{BoxedLogFormat, LogFormat, LogInfo};

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
    fn transform(&self, info: LogInfo) -> Option<LogInfo> {
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
