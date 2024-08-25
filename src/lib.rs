mod colorize;
mod combine;
mod format;
mod json;
mod log_alt;
mod printf;
mod simple;
mod timestamp;

pub use colorize::{colorize, colorize_builder};
pub use combine::combine;
pub use format::create_format;
pub use json::json;
pub use log_alt::{BoxedLogFormat, LogFormat, LogInfo};
pub use printf::printf;
pub use simple::simple;
pub use timestamp::{timestamp, timestamp_builder, TimestampOptions};
