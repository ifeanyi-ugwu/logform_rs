pub mod align;
pub mod cli;
pub mod colorize;
pub mod json;
pub mod label;
pub mod logstash;
pub mod metadata;
pub mod ms;
pub mod pad_levels;
pub mod pretty_print;
pub mod printf;
pub mod simple;
pub mod timestamp;
pub mod uncolorize;
/* chaining of formats can be achieved by the `.chain` method on the `Format`
instance hence the `combine` format is not needed  */

#[derive(Debug)]
pub enum TransformError {
    TransformationFailed,
}

pub trait Format {
    type Input;

    fn try_transform(&self, input: Self::Input) -> Result<Self::Input, TransformError>;

    fn transform(&self, input: Self::Input) -> Option<Self::Input> {
        self.try_transform(input).ok()
    }

    /*fn try_chain<F>(self, next: F) -> impl Fn(Self::Input) -> Result<Self::Input, TransformError>
    where
        Self: Sized,
        F: Format<Input = Self::Input>,
    {
        move |input| {
            let intermediate = self
                .try_transform(input)
                .map_err(|_| TransformError::TransformationFailed)?;

            next.try_transform(intermediate)
                .map_err(|_| TransformError::ChainInterrupted)
        }
    }

    fn chain<F>(self, next: F) -> impl Fn(Self::Input) -> Option<Self::Input>
    where
        Self: Sized,
        F: Format<Input = Self::Input>,
    {
        move |input| {
            self.transform(input)
                .and_then(|intermediate| next.transform(intermediate))
        }
    }*/

    fn chain<F>(self, next: F) -> ChainedFormat<Self, F>
    where
        Self: Sized,
        F: Format<Input = Self::Input>,
    {
        ChainedFormat { first: self, next }
    }
}

pub struct ChainedFormat<F1, F2> {
    first: F1,
    next: F2,
}

impl<T, F1, F2> Format for ChainedFormat<F1, F2>
where
    F1: Format<Input = T>,
    F2: Format<Input = T>,
{
    type Input = T;

    fn try_transform(&self, input: T) -> Result<T, TransformError> {
        self.first
            .try_transform(input)
            .and_then(|res| self.next.try_transform(res))
    }

    fn transform(&self, input: T) -> Option<T> {
        self.first
            .transform(input)
            .and_then(|res| self.next.transform(res))
    }
}

// Example format implementations
struct UpperCase;
impl Format for UpperCase {
    type Input = String;

    fn try_transform(&self, input: String) -> Result<Self::Input, TransformError> {
        if input.is_empty() {
            Err(TransformError::TransformationFailed)
        } else {
            Ok(input.to_uppercase())
        }
    }
}

pub struct ReverseFormat;
impl Format for ReverseFormat {
    type Input = String;

    fn try_transform(&self, input: String) -> Result<Self::Input, TransformError> {
        Ok(input.chars().rev().collect())
    }
}

struct AddSuffix(String);
impl Format for AddSuffix {
    type Input = String;

    fn try_transform(&self, input: String) -> Result<Self::Input, TransformError> {
        Ok(format!("{}{}", input, self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposed_format() {
        let upper = UpperCase;
        let reverse = ReverseFormat;
        let suffix = AddSuffix("-end".to_string());

        let format = upper.chain(reverse).chain(suffix);

        let result = format.transform("hello".to_string());

        assert_eq!(result, Some("OLLEH-end".to_string()));
    }
}
