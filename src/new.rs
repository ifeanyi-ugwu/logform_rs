pub mod align;
pub mod json;
pub mod timestamp;

pub enum TransformError {
    TransformationFailed,
    ChainInterrupted,
}

pub trait Format {
    type Input;
    type Error;

    fn try_transform(&self, input: Self::Input) -> Result<Self::Input, Self::Error>;

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

impl<T, E, F1, F2> Format for ChainedFormat<F1, F2>
where
    F1: Format<Input = T, Error = E>,
    F2: Format<Input = T, Error = E>,
{
    type Input = T;
    type Error = E;

    fn try_transform(&self, input: Self::Input) -> Result<Self::Input, Self::Error> {
        let intermediate = self.first.try_transform(input)?;
        self.next.try_transform(intermediate)
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
    type Error = ();

    fn try_transform(&self, input: String) -> Result<Self::Input, Self::Error> {
        if input.is_empty() {
            Err(())
        } else {
            Ok(input.to_uppercase())
        }
    }
}

pub struct ReverseFormat;
impl Format for ReverseFormat {
    type Input = String;
    type Error = ();

    fn try_transform(&self, input: String) -> Result<Self::Input, Self::Error> {
        Ok(input.chars().rev().collect())
    }
}

struct AddSuffix(String);
impl Format for AddSuffix {
    type Input = String;
    type Error = ();

    fn try_transform(&self, input: String) -> Result<Self::Input, Self::Error> {
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
