#![feature(crate_in_paths)]

#[macro_use]
extern crate futures;
extern crate tokio;

extern crate teleborg;

pub mod flatstream;
pub mod rawstream;
pub mod types;

pub use teleborg::Bot;

pub use self::rawstream::RawStream;
pub use self::flatstream::StreamFlatExt;

pub const TG_API_URL: &'static str = "https://api.telegram.org/bot";
