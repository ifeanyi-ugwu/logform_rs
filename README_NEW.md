# `logform`

![Crates.io](https://img.shields.io/crates/v/logform)
![Rust](https://img.shields.io/badge/rust-%E2%9C%94-brightgreen)

A flexible log format library designed for chaining and composing log transformations in Rust.

---

**Note:** The `Format` trait and all the formats below is behind the `format_trait` module, so you'll need to correct the imports:

```rust
use logform::format_trait::Format;
```

---

```rust
use logform::{align, colorize, Format, LogInfo, timestamp};

fn main() {
    let aligned_with_colors_and_time = timestamp()
        .chain(colorize())
        .chain(align());

    let mut info = LogInfo::new("info", "hi");
    info = aligned_with_colors_and_time.transform(info).unwrap();
    println!("{}", info.message);
}
```

- [`LogInfo` Objects](#loginfo-objects)
- [Understanding Formats](#understanding-formats)
  - [Chaining Formats](#chaining-formats)
  - [Filtering `LogInfo` Objects](#filtering-loginfo-objects)
- [Formats](#formats)
  - [Align](#align)
  - [CLI](#cli)
  - [Colorize](#colorize)
  - [JSON](#json)
  - [Label](#label)
  - [Logstash](#logstash)
  - [Metadata](#metadata)
  - [Ms](#ms)
  - [PadLevels](#padlevels)
  - [PrettyPrint](#prettyprint)
  - [Printf](#printf)
  - [Simple](#simple)
  - [Timestamp](#timestamp)
  - [Uncolorize](#uncolorize)

## `LogInfo` Objects

The `LogInfo` struct represents a single log message.

```rust
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}

let info = LogInfo {
    level: "info".into(),
    message: "Hey! Log something?".into(),
    meta: HashMap::new(),
};

// OR
let info = LogInfo::new("info", "Hey! Log something?");

// Add meta
let info = LogInfo::new("info", "Hey! Log something?").with_meta("key", "value"); // you can chain more

// Remove meta
info.without_meta("key");

// Get meta
info.meta.get("key");
```

Several formats in `logform` add to the meta:

| Property    | Format added by | Description                                            |
| ----------- | --------------- | ------------------------------------------------------ |
| `timestamp` | `timestamp()`   | Timestamp the message was received.                    |
| `ms`        | `ms()`          | Number of milliseconds since the previous log message. |

As a consumer, you may add whatever meta you wish.

## Understanding Formats

Formats in `logform` are structs that implement the `Format` trait, which defines a `transform` method with the signature `transform(input: Self::Input) -> Option<Self::Input>`.

- `input`: The `LogInfo` struct representing the log message.

They are expected to return one of two things:

- **A `LogInfo` Object** representing a new transformed version of the `input` argument.
- **A `None` value** indicating that the `input` argument should be ignored by the caller. (See: [Filtering `LogInfo` Objects](#filtering-loginfo-objects) below.)

Formats are designed to be simple to create. Implement the `Format` trait for your desired input type, and provide the transformation logic within the `transform` method.

```rust
use logform::{Format, LogInfo};

struct Volume {
    yell: bool,
    whisper: bool,
}

impl Format for Volume {
    type Input = LogInfo;

    fn transform(&self, mut info: LogInfo) -> Option<Self::Input> {
        if self.yell {
            info.message = info.message.to_uppercase();
        } else if self.whisper {
            info.message = info.message.to_lowercase();
        }
        Some(info)
    }
}

fn test_custom_format() {
    let scream = Volume { yell: true, whisper: false };
    let info = LogInfo::new("info", "sorry for making you YELL in your head!");
    let result = scream.transform(info).unwrap();
    println!("{}", result.message);
    // SORRY FOR MAKING YOU YELL IN YOUR HEAD!

    let whisper = Volume { yell: false, whisper: true };
    let info2 = LogInfo::new("info", "WHY ARE THEY MAKING US YELL SO MUCH!");
    let result2 = whisper.transform(info2).unwrap();
    println!("{}", result2.message);
    // why are they making us yell so much!
}
```

### Chaining Formats

Any number of formats may be chained together using the `chain` method provided by the `Format` trait. This creates a `ChainedFormat` which applies the formats in sequence.

```rust
use logform::{simple, timestamp, Format, LogInfo};

fn test_chain_formatters() {
    let combined_formatter = timestamp().chain(simple());
    let info = LogInfo::new("info", "Test message").with_meta("key", "value");
    let result = combined_formatter.transform(info).unwrap();
    println!("{}", result.message);
}
// info: Test message {"key":"value","timestamp":"2024-08-27 02:39:15"}
```

Alternatively, you can use the `chain!` macro for more concise chaining:

```rust
use logform::{simple, timestamp, LogInfo, chain};

fn test_chain_macro() {
    let combined_formatter = chain!(timestamp(), simple());
    let info = LogInfo::new("info", "Test message").with_meta("key", "value");
    let result = combined_formatter.transform(info).unwrap();
    println!("{}", result.message);
}
// info: Test message {"key":"value","timestamp":"2024-08-27 02:39:15"}
```

### Filtering `LogInfo` Objects

If you wish to filter out a given `LogInfo` Object completely, simply return `None`.

```rust
use logform::{Format, LogInfo};

struct IgnorePrivate;

impl Format for IgnorePrivate {
    type Input = LogInfo;

    fn transform(&self, info: LogInfo) -> Option<Self::Input> {
        if let Some(private) = info.meta.get("private") {
            if private == "true" {
                return None;
            }
        }
        Some(info)
    }
}

fn test_ignore_private() {
    let format = IgnorePrivate;
    let public_info = LogInfo::new("error", "Public error to share").with_meta("private", "false");
    let result = format.transform(public_info).unwrap();
    println!("{}", result.message);
    // Public error to share

    let private_info = LogInfo::new("error", "This is super secret - hide it.").with_meta("private", "true");
    let result = format.transform(private_info);
    println!("{:?}", result);
    // None
}
```

Chaining formats using `chain` will respect any `None` values returned and stop the evaluation of later formats in the chain.

```rust
use logform::{Format, LogInfo};

struct NeverReached;

impl Format for NeverReached {
    type Input = LogInfo;
    fn transform(&self, _info: LogInfo) -> Option<Self::Input> {
        panic!("Never reached");
    }
}

fn test_chain_with_none() {
    let will_never_panic = Format::chain(None::<LogInfo>, NeverReached);
    let info = LogInfo::new("info", "wow such testing");
    println!("{:?}", will_never_panic.transform(info));
    // None
}
```

## Formats

### Align

The `align` format adds a tab character before the message.

```rust
let aligned_format = align();
```

### CLI

The `cli` format is a combination of the `colorize` and `pad_levels` formats. It pads, colorizes, and then formats the log message as `level:message`.

```rust
use logform::{cli, LogInfo};
use colored::control::set_override;

fn test_cli() {
    set_override(true);
    let cli_format = cli();
    let info = LogInfo::new("info", "my message");
    let transformed_info = cli_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

### Colorize

The `colorize` format adds colors to log levels and messages.

```rust
use logform::{colorize, LogInfo};
use colored::control::set_override;

fn test_colorize() {
    set_override(true);
    let colorizer = colorize();
    let info = LogInfo::new("info", "Info message");
    let result = colorizer.transform(info).unwrap();
    println!("{}", result.message);
}
```

### JSON

The `json` format converts the log info into a JSON string.

```rust
use logform::{json, LogInfo};

fn test_json() {
    let json_format = json();
    let info = LogInfo::new("info", "User logged in")
        .with_meta("user_id", 12345)
        .with_meta("session_id", "abcde12345");
    let result = json_format.transform(info).unwrap();
    println!("{}", result.message);
}
```

### Label

The `label` format adds a specified label to the log message or metadata.

```rust
use logform::{label, LogInfo};

fn test_label_message() {
    let label_format = label().with_label("MY_LABEL").with_message(true);
    let info = LogInfo::new("info", "Test message");
    let result = label_format.transform(info).unwrap();
    println!("{}", result.message);
}

fn test_label_meta() {
    let label_format = label().with_label("MY_LABEL").with_message(false);
    let info = LogInfo::new("info", "Test message");
    let result = label_format.transform(info).unwrap();
    println!("{:?}", result.meta);
}
```

### Logstash

The `logstash` format converts the log info into a Logstash-compatible JSON string.

```rust
use logform::{logstash, timestamp, LogInfo};

fn test_logstash() {
    let logstash_format = timestamp().chain(logstash());
    let info = LogInfo::new("info", "my message");
    let formatted_info = logstash_format.transform(info).unwrap();
    println!("{}", formatted_info.message);
}
```

### Metadata

The `metadata` format collects metadata from the log and adds it to the specified key.

```rust
use logform::{metadata, LogInfo};
use serde_json::json;

fn test_metadata() {
    let metadata_format = metadata();
    let mut info = LogInfo::new("info", "Test message");
    info.meta.insert("key1".to_string(), json!("value1"));
    info.meta.insert("key2".to_string(), json!("value2"));
    let result = metadata_format.transform(info).unwrap();
    println!("{:?}", result.meta);
}
```

### Ms

The `ms` format adds the time in milliseconds since the last log message.

```rust
use logform::{ms, LogInfo};

fn test_ms() {
    let ms_format = ms();
    let info = LogInfo::new("info", "my message");
    let formatted_info = ms_format.transform(info).unwrap();
    println!("{:?}", formatted_info.meta.get("ms"));
}
```

### PadLevels

The `pad_levels` format pads levels to be the same length.

```rust
use logform::{pad_levels, LogInfo};

fn test_pad_levels() {
    let pad_levels_format = pad_levels();
    let info = LogInfo::new("info", "my message");
    let transformed_info = pad_levels_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

### PrettyPrint

The `pretty_print` format provides a more readable output of the log info.

```rust
use logform::{pretty_print, LogInfo};

fn test_pretty_print() {
    let pretty_format = pretty_print().with_option("colorize", "true");
    let info = LogInfo::new("info", "my message");
    let transformed_info = pretty_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

### Printf

The `printf` format allows you to define a custom formatting function.

```rust
use logform::{printf, LogInfo};

fn test_printf() {
    let printf_format = printf(|info| {
        format!("{} - {}: {}", info.meta_as_str("timestamp").unwrap_or(""), info.level, info.message)
    });
    let mut info = LogInfo::new("info", "my message");
    info.meta.insert("timestamp".to_string(), serde_json::Value::String("2024-01-01 12:00:00".to_string()));
    let transformed_info = printf_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

### Simple

The `simple` format provides a basic string representation of the log info.

```rust
use logform::{simple, LogInfo};

fn test_simple() {
    let simple_format = simple();
    let info = LogInfo::new("info", "my message");
    let transformed_info = simple_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

### Timestamp

The `timestamp` format adds a timestamp to the log info.

```rust
use logform::{timestamp, LogInfo};

fn test_timestamp() {
    let timestamp_format = timestamp().with_option("format", "%Y-%m-%d %H:%M:%S");
    let info = LogInfo::new("info", "my message");
    let transformed_info = timestamp_format.transform(info).unwrap();
    println!("{:?}", transformed_info.meta.get("timestamp"));
}
```

### Uncolorize

The `uncolorize` format removes ANSI color codes from the log info.

```rust
use logform::{uncolorize, LogInfo};
use colored::control::set_override;

fn test_uncolorize() {
    set_override(true);
    let uncolorize_format = uncolorize();
    let mut info = LogInfo::new("info", "\x1b[34mmy message\x1b[0m");
    info.level = "\x1b[34minfo\x1b[0m".to_string();
    let transformed_info = uncolorize_format.transform(info).unwrap();
    println!("{}", transformed_info.message);
}
```

## Usage

To use logform in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
logform = "0.3"
```

or with

```bash
cargo add logform
```

Then, in your Rust code:

```rust
use logform::{LogInfo, timestamp, json};

fn main() {
    let format = timestamp().chain(json());
    let info = LogInfo::new("info", "Test message");
    let formatted_info = format.transform(info).unwrap();
    println!("{}", formatted_info.message);
}
```

## Testing

Run the tests using:

```bash
cargo test
```

## License

This project is licensed under the MIT License.

## Acknowledgements

This library is inspired by the [logform](https://github.com/winstonjs/logform) package for Node.js.
