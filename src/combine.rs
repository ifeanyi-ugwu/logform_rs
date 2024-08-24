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
    fn transform(&self, info: &mut LogInfo) {
        for format in &self.formats {
            format.transform(info);
        }
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
