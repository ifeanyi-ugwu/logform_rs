use crate::log_alt::{BoxedLogFormat, LogFormat, LogInfo};

pub struct CombineFormat {
    formats: Vec<BoxedLogFormat>,
}

impl CombineFormat {
    pub fn new(formats: Vec<BoxedLogFormat>) -> Self {
        Self { formats }
    }
}

impl LogFormat for CombineFormat {
    fn transform(&self, info: LogInfo) -> Option<LogInfo> {
        let mut current_info = info;

        for format in &self.formats {
            if let Some(new_info) = format.transform(current_info) {
                current_info = new_info;
            } else {
                return None;
            }
        }

        Some(current_info)
    }
}
/*
pub fn combine(formats: Vec<Box<dyn LogFormat>>) -> CombineFormat {
    CombineFormat::new(formats)
}
*/
pub fn combine(formats: Vec<BoxedLogFormat>) -> BoxedLogFormat {
    Box::new(CombineFormat::new(formats))
}
