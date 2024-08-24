use crate::log_alt::{LogFormat, LogInfo};

pub struct CombineFormat {
    formats: Vec<Box<dyn LogFormat>>,
}

impl CombineFormat {
    pub fn new(formats: Vec<Box<dyn LogFormat>>) -> Self {
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
pub fn combine(formats: Vec<Box<dyn LogFormat>>) -> Box<dyn LogFormat> {
    Box::new(CombineFormat::new(formats))
}
