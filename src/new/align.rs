use super::Format;

pub struct AlignFormat;

impl Format for AlignFormat {
    type Input = String;
    type Error = (); // No real errors, so we use an empty tuple

    fn try_transform(&self, input: String) -> Result<Self::Input, Self::Error> {
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
