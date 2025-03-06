use super::Format;

pub struct AlignFormat;

impl Format for AlignFormat {
    type Input = String;

    fn transform(&self, input: String) -> Option<Self::Input> {
        Some(format!("\t{}", input))
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
