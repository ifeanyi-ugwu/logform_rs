pub mod align;
pub mod cli;
pub mod colorize;
pub mod json;
pub mod label;
pub mod logstash;
mod macros;
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

pub trait Format {
    type Input;

    fn transform(&self, input: Self::Input) -> Option<Self::Input>;

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

    fn transform(&self, input: String) -> Option<Self::Input> {
        if input.is_empty() {
            None
        } else {
            Some(input.to_uppercase())
        }
    }
}

pub struct ReverseFormat;
impl Format for ReverseFormat {
    type Input = String;

    fn transform(&self, input: String) -> Option<Self::Input> {
        Some(input.chars().rev().collect())
    }
}

#[derive(Clone)]
struct AddSuffix(String);
impl Format for AddSuffix {
    type Input = String;

    fn transform(&self, input: String) -> Option<Self::Input> {
        Some(format!("{}{}", input, self.0))
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

    // Implement volume control formats
    struct Yell;
    impl Format for Yell {
        type Input = String;

        fn transform(&self, input: String) -> Option<Self::Input> {
            Some(input.to_uppercase())
        }
    }

    struct Whisper;
    impl Format for Whisper {
        type Input = String;

        fn transform(&self, input: String) -> Option<Self::Input> {
            Some(input.to_lowercase())
        }
    }

    struct VolumeControlFormat<F> {
        inner: F,
        volume: VolumeLevel,
    }

    #[derive(Debug, PartialEq)]
    enum VolumeLevel {
        Normal,
        Loud,
        Quiet,
    }

    impl<F> Format for VolumeControlFormat<F>
    where
        F: Format<Input = String>,
    {
        type Input = String;

        fn transform(&self, input: String) -> Option<Self::Input> {
            match self.volume {
                VolumeLevel::Normal => self.inner.transform(input),
                VolumeLevel::Loud => Yell.transform(input),
                VolumeLevel::Quiet => Whisper.transform(input),
            }
        }
    }

    #[test]
    fn test_volume_control() {
        // Create a base format (could be any format)
        let base_format = AddSuffix("!".to_string());

        // Test normal volume
        let normal_volume = VolumeControlFormat {
            inner: base_format.clone(),
            volume: VolumeLevel::Normal,
        };
        let result = normal_volume.transform("hello".to_string());
        assert_eq!(result, Some("hello!".to_string()));

        // Test loud volume (uppercase)
        let loud_volume = VolumeControlFormat {
            inner: base_format.clone(),
            volume: VolumeLevel::Loud,
        };
        let result = loud_volume.transform("hello".to_string());
        assert_eq!(result, Some("HELLO!".to_string()));

        // Test quiet volume (lowercase)
        let quiet_volume = VolumeControlFormat {
            inner: base_format,
            volume: VolumeLevel::Quiet,
        };
        let result = quiet_volume.transform("HELLO".to_string());
        assert_eq!(result, Some("hello!".to_string()));

        // Demonstrate chaining
        let complex_format = normal_volume
            .chain(Yell)
            .chain(AddSuffix("-processed".to_string()));
        let result = complex_format.transform("hello".to_string());
        assert_eq!(result, Some("HELLO!-processed".to_string()));
    }

    #[test]
    fn test_volume_formats() {
        let yell = Yell;
        let whisper = Whisper;

        let yelled = yell.transform("hello".to_string());
        assert_eq!(yelled, Some("HELLO".to_string()));

        let whispered = whisper.transform("HELLO".to_string());
        assert_eq!(whispered, Some("hello".to_string()));

        // Chaining example
        let chained_format = Whisper.chain(Yell);
        let result = chained_format.transform("HeLLo".to_string());
        assert_eq!(result, Some("HELLO".to_string()));
    }

    pub struct IgnorePrivate;
    impl Format for IgnorePrivate {
        type Input = String;

        fn transform(&self, input: String) -> Option<Self::Input> {
            Some(input)
        }
    }

    // To handle more complex private message filtering, we'll create a struct
    struct LogMessage {
        message: String,
        is_private: bool,
    }

    struct IgnorePrivateFormat;
    impl Format for IgnorePrivateFormat {
        type Input = LogMessage;

        fn transform(&self, input: LogMessage) -> Option<Self::Input> {
            if input.is_private {
                None
            } else {
                Some(input)
            }
        }
    }

    #[test]
    fn test_ignore_private() {
        let filter = IgnorePrivateFormat;

        let public_message = LogMessage {
            message: "Public message".to_string(),
            is_private: false,
        };

        let private_message = LogMessage {
            message: "Secret message".to_string(),
            is_private: true,
        };

        // Public message passes through
        let result = filter.transform(public_message);
        assert!(result.is_some());
        assert_eq!(result.unwrap().message, "Public message");

        // Private message is filtered out
        let result = filter.transform(private_message);
        assert!(result.is_none());

        // Chaining example
        /*let chained_filter = IgnorePrivateFormat.chain(Yell);
        let public_message = LogMessage {
            message: "hello".to_string(),
            is_private: false,
        };*/

        /*  let result = chained_filter.transform(public_message);
        assert!(result.is_some());
        assert_eq!(result.unwrap().message, "HELLO");
        */
    }
}
