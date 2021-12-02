#![warn(clippy::all)]

use std::error::Error;

pub type Res<T> = Result<T, Box<dyn Error>>;

mod io;
mod pow;
mod ssl;

pub mod error;
pub mod protocol;
pub mod userdata;

pub use error::Err as ExasolErr;
