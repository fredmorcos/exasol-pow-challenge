#![warn(clippy::all)]

use std::io::{self, Read};

pub(crate) fn read_until<R: Read>(
  reader: &mut R,
  data: &mut Vec<u8>,
  pred: impl Fn(u8) -> bool,
) -> io::Result<()> {
  let mut buffer = [0_u8; 1024];

  data.clear();

  loop {
    let bytes = reader.read(&mut buffer)?;

    if bytes == 0 {
      break;
    }

    for &byte in &buffer[..bytes] {
      if pred(byte) {
        return Ok(());
      }

      data.push(byte);
    }
  }

  Ok(())
}
