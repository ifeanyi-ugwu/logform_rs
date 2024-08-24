mod colorize;
mod combine;
mod json;
mod log_alt;
mod printf;
mod simple;
mod timestamp;

pub use colorize::colorize;
pub use combine::combine;
pub use json::json;
pub use log_alt::{LogFormat, LogInfo};
pub use printf::printf;
pub use simple::simple;
pub use timestamp::{timestamp, TimestampOptions};
