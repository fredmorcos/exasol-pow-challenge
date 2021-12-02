#![warn(clippy::all)]

use std::error::Error;

pub type Res<T> = Result<T, Box<dyn Error>>;

pub mod ssl;
mod io;

pub mod error;
pub mod userdata;

pub use error::Err as ExasolErr;
