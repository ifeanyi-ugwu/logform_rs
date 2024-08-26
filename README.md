# logform-rs

A flexible log formatting library for Rust, inspired by the Node.js logform package.

```rust
use logform::{combine, colorize, json, printf, timestamp, LogInfo};

let format = combine(vec![
    timestamp(),
    colorize().with_option("colors", r#"{"info": ["blue"]}"#),
    printf(|info| format!("{} - {}: {}", info.meta["timestamp"], info.level, info.message)),
]);

let mut info = LogInfo::new("info", "Hello, logform-rs!");
let formatted_info = format.transform(info, None).unwrap();
println!("{}", formatted_info.message);
```

## Features

- Composable log formats
- Various built-in formats
- Extensible with custom formats
- Chainable options for each format

## Table of Contents

- [LogInfo Objects](#loginfo-objects)
- [Formats](#formats)
  - [Align](#align)
  - [Colorize](#colorize)
  - [Combine](#combine)
  - [JSON](#json)
  - [Ms](#ms)
  - [PrettyPrint](#prettyprint)
  - [Printf](#printf)
  - [Simple](#simple)
  - [Timestamp](#timestamp)
  - [Uncolorize](#uncolorize)

## LogInfo Objects

The `LogInfo` struct represents a single log message:

```rust
pub struct LogInfo {
    pub level: String,
    pub message: String,
    pub meta: HashMap<String, Value>,
}
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
logform = "0.1.0"
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
