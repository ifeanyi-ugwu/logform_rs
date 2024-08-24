use crate::log_alt::{LogFormat, LogInfo};

pub struct PrintfFormat<F: Fn(&LogInfo) -> String> {
    formatter: F,
}

impl<F> PrintfFormat<F>
where
    F: Fn(&LogInfo) -> String,
{
    pub fn new(formatter: F) -> Self {
        Self { formatter }
    }
}

impl<F> LogFormat for PrintfFormat<F>
where
    F: Fn(&LogInfo) -> String,
{
    fn transform(&self, info: &mut LogInfo) {
        info.message = (self.formatter)(info);
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

pub fn printf<F>(formatter: F) -> Box<dyn LogFormat>
where
    F: Fn(&LogInfo) -> String + 'static,
{
    Box::new(PrintfFormat::new(formatter))
}
