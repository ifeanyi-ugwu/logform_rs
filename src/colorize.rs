use crate::log_alt::{LogFormat, LogInfo};

pub struct ColorizeFormat {
    color: String,
}

impl ColorizeFormat {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
        }
    }
}

impl LogFormat for ColorizeFormat {
    fn transform(&self, info: &mut LogInfo) {
        info.level = format!("\x1b[{}m{}\x1b[0m", self.color, info.level);
    }
}

pub fn colorize(color: &str) -> ColorizeFormat {
    ColorizeFormat::new(color)
}
