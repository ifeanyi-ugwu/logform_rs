use super::{Format, TransformError};

pub struct AlignFormat;

impl Format for AlignFormat {
    type Input = String;

    fn try_transform(&self, input: String) -> Result<Self::Input, TransformError> {
        Ok(format!("\t{}", input))
    }
}

pub fn align() -> AlignFormat {
    AlignFormat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_format() {
        let align = AlignFormat;

        let result = align.transform("Test message".to_string());
        assert_eq!(result, Some("\tTest message".to_string()));
    }
}
