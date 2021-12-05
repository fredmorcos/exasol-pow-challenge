#![warn(clippy::all)]

use crate::error::Err;
use crate::pow::pow;
use crate::ssl::create_ssl_stream;
use crate::Res;
use log::debug;
use openssl::ssl::SslStream;
use std::{io::Write, marker::PhantomData, net::TcpStream, path::Path, str::Split};

pub trait State {}
impl State for () {}

pub struct StateHelo;
impl State for StateHelo {}

pub struct StatePow;
impl State for StatePow {}

pub struct Data;
impl State for Data {}

pub struct Exasol<S: State = ()> {
  stream: SslStream<TcpStream>,
  buffer: Vec<u8>,
  phantom: PhantomData<S>,
}

impl<S1: State> Exasol<S1> {
  fn make<S0: State>(old: Exasol<S0>) -> Self {
    Self { stream: old.stream, buffer: old.buffer, phantom: PhantomData }
  }

  fn get_command_and_args(&mut self) -> Res<(&str, Split<char>)> {
    crate::io::read_until(&mut self.stream, &mut self.buffer, |b| b == b'\n')?;
    let buffer_str = std::str::from_utf8(&self.buffer)?;
    let mut args = buffer_str.trim().split(' ');
    let command = args.next().ok_or(Err::CommandExpected)?;

    if command == "ERROR" {
      let msg = args.collect::<Vec<&str>>().join(" ");
      return Err::server(msg);
    }

    Ok((command, args))
  }
}

impl Exasol {
  pub fn new(cert_file: &Path, keylog_file: Option<&Path>, address: &str) -> Res<Self> {
    let stream = create_ssl_stream(cert_file, keylog_file, address)?;
    Ok(Self { stream, buffer: vec![], phantom: PhantomData })
  }

  pub fn connect(mut self) -> Res<Exasol<StateHelo>> {
    self.stream.connect()?;
    debug!("SSL stream connected");
    Ok(Exasol::make(self))
  }
}

impl Exasol<StateHelo> {
  pub fn handshake(mut self) -> Res<Exasol<StatePow>> {
    let (command, _) = self.get_command_and_args()?;

    if command != "HELO" {
      return Err::unknown_or_unexpected_command(command, "HELO");
    }

    self.stream.write_all(b"EHLO\n")?;
    self.stream.flush()?;

    Ok(Exasol::make(self))
  }
}

impl Exasol<StatePow> {
  pub fn pow(mut self) -> Res<Exasol<Data>> {
    let (command, mut args) = self.get_command_and_args()?;

    if command != "POW" {
      return Err::unknown_or_unexpected_command(command, "POW");
    }

    let authdata = args.next().ok_or(Err::MissingArg)?;
    let difficulty = {
      let val = args.next().ok_or(Err::MissingArg)?.parse::<usize>()?;
      if val > 9 {
        return Err::invalid_difficulty(val);
      }
      val
    };

    debug!("Authdata = {}  |  Difficulty = {}", authdata, difficulty);

    let random_bytes = pow(authdata, difficulty)?.ok_or(Err::CannotPow)?;
    let random_string = std::str::from_utf8(&random_bytes)?;

    assert!(!random_bytes.is_empty());

    self.stream.write_all(random_string.as_bytes())?;
    self.stream.write_all(b"\n")?;
    self.stream.flush()?;

    Ok(Exasol::make(self))
  }
}
