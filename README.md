# logform-rs

A flexible log formatting library for Rust, inspired by the Node.js logform package.

```rust
use logform::{combine, colorize_builder, json, printf, timestamp, LogInfo};

let format = combine(vec![
    timestamp(None),
    colorize_builder().add_color("info", "green").build(),
    printf(|info| format!("{} - {}: {}", info.meta["timestamp"], info.level, info.message)),
]);

let mut info = LogInfo::new("info", "Hello, logform-rs!");
format.transform(&mut info);
println!("{}", info.message);
```

## Features

- Composable log formats
- Various built-in formats
- Extensible with custom formats

## Table of Contents

- [LogInfo Objects](#loginfo-objects)
- [Formats](#formats)
  - [Colorize](#colorize)
  - [Combine](#combine)
  - [JSON](#json)
  - [Printf](#printf)
  - [Simple](#simple)
  - [Timestamp](#timestamp)

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

### Colorize

The `colorize` format adds colors to log levels and messages.

```rust
let colorizer = colorize_builder()
    .add_color("info", "green")
    .add_color("warn", "yellow")
    .add_color("error", "red")
    .set_all(true)
    .build();
```

### Combine

The `combine` format allows you to chain multiple formats together.

```rust
let combined_format = combine(vec![
    timestamp(None),
    json(),
    colorize_builder().add_color("info", "blue").build(),
]);
```

### JSON

The `json` format converts the log info into a JSON string.

```rust
let json_format = json();
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
let timestamp_format = timestamp_builder()
    .format("%Y-%m-%d %H:%M:%S")
    .alias("time")
    .build();
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
    timestamp(None),
    json(),
]);

let mut log_info = LogInfo::new("info", "Test message");
format.transform(&mut log_info);
println!("{}", log_info.message);
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
