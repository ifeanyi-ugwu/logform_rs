# `logform`

![Crates.io](https://img.shields.io/crates/v/logform)
![Rust](https://img.shields.io/badge/rust-%E2%9C%94-brightgreen)

A flexible log format library designed for chaining and composing log transformations in Rust.

```rust
use logform::{align, colorize, combine, printf, timestamp, LogInfo};

pub fn initialize_and_test_formats() {
    let aligned_with_colors_and_time = combine(vec![
        colorize(),
        timestamp(),
        align(),
        printf(|info| {
            format!(
                "{} {}: {}",
                info.meta["timestamp"], info.level, info.message
            )
        }),
    ]);

    let mut info = LogInfo::new("info", "hi");
    info = aligned_with_colors_and_time.transform(info, None).unwrap();
    println!("{}", info.message);
}
```

- [`LogInfo` Objects](#loginfo-objects)
- [Understanding Formats](#understanding-formats)
  - [Combining Formats](#combining-formats)
  - [Filtering `LogInfo` Objects](#filtering-loginfo-objects)
- [Formats](#formats)
  - [Align](#align)
  - [Colorize](#colorize)
  - [Combine](#combine)
  - [JSON](#json)
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
    level: "info".into(),                  // Level of the logging message
    message: "Hey! Log something?".into(), // Descriptive message being logged
    meta: HashMap::new(),                   // Other properties
};

//OR
let info = LogInfo::new("info", "Hey! Log something?");

//add meta
let info = LogInfo::new("info", "Hey! Log something?").add_meta("key", "value");//you can chain more

//remove meta
info.remove_meta("key");

//get meta
info.get_meta("key");
```

Several of the formats in `logform` itself add to the meta:

| Property    | Format added by | Description                                            |
| ----------- | --------------- | ------------------------------------------------------ |
| `timestamp` | `timestamp()`   | Timestamp the message was received.                    |
| `ms`        | `ms()`          | Number of milliseconds since the previous log message. |

As a consumer, you may add whatever meta you wish

## Understanding Formats

Formats in `logform` are structs that implement a `transform` method with the signature `transform(info: LogInfo, opts: FormatOptions) -> Option<LogInfo>`.

- `info`: The LogInfo struct representing the log message.
- `opts`: Settings(Options) specific to the current instance of the format.

They are expected to return one of two things:

- **A `LogInfo` Object** representing a new transformed version of the `info` argument. The LogInfo struct is treated as immutable, meaning a new instance is created and returned with the desired modifications.
- **A None value** indicating that the `info` argument should be ignored by the caller. (See: [Filtering `LogInfo` Objects](#filtering-loginfo-objects)) below.

Creating formats is designed to be as simple as possible. To define a new format, use `Format::new()` and pass a closure that implements the transformation logic:`transform(info: LogInfo, opts: FormatOptions)`.

The named `Format` returned can be used to create as many copies of the given `Format` as desired:

```rust
use logform::Format;

fn test_custom_format() {
    let volume = Format::new(|mut info: LogInfo, opts: FormatOptions| {
        if let Some(opts) = opts {
            if opts.get("yell").is_some() {
                info.message = info.message.to_uppercase();
            } else if opts.get("whisper").is_some() {
                info.message = info.message.to_lowercase();
            }
        }
        Some(info)
    });

    // `volume` is now a Format instance that can be used for transformations
    let mut scream_opts = HashMap::new();
    scream_opts.insert("yell".to_string(), "true".to_string());
    let scream = volume.clone();

    let info = LogInfo::new("info", "sorry for making you YELL in your head!");

    let result = scream.transform(info, Some(scream_opts)).unwrap();
    println!("{}", result.message);
    //SORRY FOR MAKING YOU YELL IN YOUR HEAD!

    // `volume` can be used multiple times with different options
    let mut whisper_opts = HashMap::new();
    whisper_opts.insert("whisper".to_string(), "true".to_string());
    let whisper = volume;

    let info2 = LogInfo::new("info", "WHY ARE THEY MAKING US YELL SO MUCH!");

    let result2 = whisper.transform(info2, Some(whisper_opts)).unwrap();
    println!("{}", result2.message);
    //why are they making us yell so much!
}

```

### Combining Formats

Any number of formats may be combined into a single format using `logform::combine`. Since `logform::combine` takes no options, it returns a pre-created instance of the combined format.

```rust
use logform::{combine, simple, timestamp};

fn test_combine_formatters() {
    // Combine timestamp and simple
    let combined_formatter = combine(vec![timestamp(), simple()]);

    let info = LogInfo::new("info", "Test message").add_meta("key", "value");

    let result = combined_formatter.transform(info, None).unwrap();
    println!("{}", result.message);
}
//info: Test message {"key":"value","timestamp":"2024-08-27 02:39:15"}
```

### Filtering `LogInfo` Objects

If you wish to filter out a given `LogInfo` Object completely, simply return `None`.

```rust
use logform::Format;

fn test_ignore_private() {
    let ignore_private = Format::new(|info: LogInfo, _opts: FormatOptions| {
        if let Some(private) = info.meta.get("private") {
            if private == "true" {
                return None;
            }
        }
        Some(info)
    });

    let format = ignore_private;

    let public_info = LogInfo::new("error", "Public error to share").add_meta("private", "false");

    let result = format.transform(public_info, None).unwrap();
    println!("{}", result.message);
    //Public error to share

    let private_info =
        LogInfo::new("error", "This is super secret - hide it.").add_meta("private", "true");

    let result = format.transform(private_info, None);
    println!("{:?}", result);
    // None
}
```

The use of `logform::combine` will respect any `None` values returned and stop the evaluation of later formats in the series. For example:

```rust
use logform::{combine, Format};

let will_never_panic = combine(vec![
    Format::new(|_info, _opts| None), // Ignores everything
    Format::new(|_info, _opts| {
        panic!("Never reached");
    }),
]);

let info = LogInfo::new("info", "wow such testing");

println!("{:?}", will_never_panic.transform(info, None));
// None
```

## Formats

### Align

The `align` format adds a tab character before the message.

```rust
let aligned_format = align();
```

### Colorize

The `colorize` format adds colors to log levels and messages.

```rust
let colorizer = colorize()
    .with_option("colors", r#"{"info": ["blue"], "error": ["red", "bold"]}"#)
    .with_option("all", "true");
```

### Combine

The `combine` format allows you to chain multiple formats together.

```rust
let combined_format = combine(vec![
    timestamp(),
    json(),
    colorize().with_option("colors", r#"{"info": ["blue"]}"#),
]);
```

### JSON

The `json` format converts the log info into a JSON string.

```rust
let json_format = json();
```

### Ms

The `ms` format adds the time in milliseconds since the last log message.

```rust
let ms_format = ms();
```

### PrettyPrint

The `pretty_print` format provides a more readable output of the log info.

```rust
let pretty_format = pretty_print().with_option("colorize", "true");
```

### Printf

The `printf` format allows you to define a custom formatting function.

```rust
let printf_format = printf(|info| {
    format!("{} - {}: {}", info.meta["timestamp"], info.level, info.message)
});
```

### Simple

The `simple` format provides a basic string representation of the log info.

```rust
let simple_format = simple();
```

### Timestamp

The `timestamp` format adds a timestamp to the log info.

```rust
let timestamp_format = timestamp()
    .with_option("format", "%Y-%m-%d %H:%M:%S")
    .with_option("alias", "log_time");
```

### Uncolorize

The `uncolorize` format removes ANSI color codes from the log info.

```rust
let uncolorize_format = uncolorize();
```

## Usage

To use logform-rs in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
logform = "0.1.4"
```

Then, in your Rust code:

```rust
use logform::{LogInfo, combine, timestamp, json};

let format = combine(vec![
    timestamp(),
    json(),
]);

let info = LogInfo::new("info", "Test message");
let formatted_info = format.transform(info, None).unwrap();
println!("{}", formatted_info.message);
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
