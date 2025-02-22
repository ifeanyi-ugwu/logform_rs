use super::{Format, TransformError};
use crate::LogInfo;
use serde_json::json;

pub struct LabelFormat {
    label: String,
    message: bool,
}

impl LabelFormat {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            message: false,
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn with_message(mut self, apply: bool) -> Self {
        self.message = apply;
        self
    }
}

impl Format for LabelFormat {
    type Input = LogInfo;

    fn try_transform(&self, mut info: LogInfo) -> Result<Self::Input, TransformError> {
        if self.message {
            info.message = format!("[{}] {}", self.label, info.message);
        } else {
            info.meta
                .insert("label".to_string(), json!(self.label.clone()));
        }
        Ok(info)
    }
}

pub fn label() -> LabelFormat {
    LabelFormat::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_format_message() {
        let label_format = LabelFormat::new().with_label("MY_LABEL").with_message(true);
        let info = LogInfo::new("info", "Test message");
        let result = label_format.transform(info).unwrap();
        assert_eq!(result.message, "[MY_LABEL] Test message");
    }

    #[test]
    fn test_label_format_meta() {
        let label_format = LabelFormat::new()
            .with_label("MY_LABEL")
            .with_message(false);
        let info = LogInfo::new("info", "Test message");
        let result = label_format.transform(info).unwrap();
        assert_eq!(result.meta.get("label"), Some(&json!("MY_LABEL")));
    }
}
