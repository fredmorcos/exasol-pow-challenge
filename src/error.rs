#![warn(clippy::all)]

use crate::Res;
use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display)]
#[display(fmt = "{}")]
pub enum Err {
  #[display(fmt = "Expecting a command")]
  CommandExpected,
  #[display(fmt = "Server error: {}", _0)]
  Server(String),
  #[display(fmt = "Unknown or unexpected command `{}`, expecting `{}`", _0, _1)]
  UnknownUnexpectedCommand(String, String),
  #[display(fmt = "Missing argument")]
  MissingArg,
  #[display(fmt = "Invalid difficulty `{}`", _0)]
  InvalidDifficulty(usize),
  #[display(fmt = "Could not find a random string")]
  CannotPow,
}

impl Err {
  pub(crate) fn server<T>(msg: String) -> Res<T> {
    Err(Box::new(Err::Server(msg)))
  }

  pub(crate) fn unknown_or_unexpected_command<T>(command: &str, expected: &str) -> Res<T> {
    Err(Box::new(Err::UnknownUnexpectedCommand(command.to_string(), expected.to_string())))
  }

  pub(crate) fn invalid_difficulty<T>(difficulty: usize) -> Res<T> {
    Err(Box::new(Err::InvalidDifficulty(difficulty)))
  }
}

impl Error for Err {}
