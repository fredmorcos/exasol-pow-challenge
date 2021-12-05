#![warn(clippy::all)]

use crate::pow::pow;
use crate::ssl::create_ssl_stream;
use crate::Res;
use crate::{error::Err, userdata::UserData};
use log::{debug, info};
use openssl::ssl::SslStream;
use sha1::{Digest, Sha1};
use std::{io::Write, net::TcpStream, path::Path, str::Split};

pub trait State: Default {}
impl State for () {}

#[derive(Default)]
pub struct StateHelo;
impl State for StateHelo {}

#[derive(Default)]
pub struct StatePow;
impl State for StatePow {}

#[derive(Default)]
pub struct StateEnd;
impl State for StateEnd {}

#[derive(Default)]
pub struct StateData {
  got_mailnum: bool,
  got_addrnum: bool,
  hasher: Sha1,
}
impl State for StateData {}

impl StateData {
  pub fn new(authdata: &str) -> Self {
    let mut hasher = Sha1::new();
    hasher.update(authdata);
    Self { got_mailnum: false, got_addrnum: false, hasher }
  }

  /// Get state's mailnum status.
  pub fn got_mailnum(&self) -> bool {
    self.got_mailnum
  }

  /// Set the state's mailnum status to true.
  pub fn set_got_mailnum(&mut self) {
    self.got_mailnum = true;
  }

  /// Get the state's addrnum status.
  pub fn got_addrnum(&self) -> bool {
    self.got_addrnum
  }

  /// Set the state's addrnum status to true.
  pub fn set_got_addrnum(&mut self) {
    self.got_addrnum = true;
  }

  pub fn hash(&self, data: &str) -> [u8; 20] {
    let mut hasher = self.hasher.clone();
    hasher.update(data);
    hasher.finalize().into()
  }
}

pub struct Exasol<S: State = ()> {
  stream: SslStream<TcpStream>,
  buffer: Vec<u8>,
  state: S,
}

impl<S1: State> Exasol<S1> {
  fn make<S0: State>(old: Exasol<S0>) -> Self {
    Self::make_with_state(old, Default::default())
  }

  fn make_with_state<S0: State>(old: Exasol<S0>, new_state: S1) -> Self {
    Self { stream: old.stream, buffer: old.buffer, state: new_state }
  }
}

impl<S: State> Exasol<S> {
  fn get_command_args_and_state(&mut self) -> Res<(&str, Split<char>, &S)> {
    crate::io::read_until(&mut self.stream, &mut self.buffer, |b| b == b'\n')?;
    let buffer_str = std::str::from_utf8(&self.buffer)?;
    let mut args = buffer_str.trim().split(' ');
    let command = args.next().ok_or(Err::CommandExpected)?;

    if command == "ERROR" {
      let msg = args.collect::<Vec<&str>>().join(" ");
      return Err::server(msg);
    }

    Ok((command, args, &self.state))
  }
}

impl Exasol {
  pub fn new(cert_file: &Path, keylog_file: Option<&Path>, address: &str) -> Res<Self> {
    let stream = create_ssl_stream(cert_file, keylog_file, address)?;
    Ok(Self { stream, buffer: vec![], state: Default::default() })
  }

  pub fn connect(mut self) -> Res<Exasol<StateHelo>> {
    self.stream.connect()?;
    debug!("SSL stream connected");
    Ok(Exasol::make(self))
  }
}

impl Exasol<StateHelo> {
  pub fn handshake(mut self) -> Res<Exasol<StatePow>> {
    let (command, _, _) = self.get_command_args_and_state()?;

    if command != "HELO" {
      return Err::unknown_or_unexpected_command(command, "HELO");
    }

    self.stream.write_all(b"EHLO\n")?;
    self.stream.flush()?;

    Ok(Exasol::make(self))
  }
}

impl Exasol<StatePow> {
  pub fn pow(mut self) -> Res<Exasol<StateData>> {
    let (command, mut args, _) = self.get_command_args_and_state()?;

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

    // We create the new state here to avoid an authdata clone.
    let new_state = StateData::new(authdata);

    debug!("Authdata = {}  |  Difficulty = {}", authdata, difficulty);

    let random_bytes = pow(authdata, difficulty)?.ok_or(Err::CannotPow)?;
    let random_string = std::str::from_utf8(&random_bytes)?;

    assert!(!random_bytes.is_empty());
    assert!(!random_string.is_empty());

    self.stream.write_all(random_string.as_bytes())?;
    self.stream.write_all(b"\n")?;
    self.stream.flush()?;

    Ok(Exasol::make_with_state(self, new_state))
  }
}

impl Exasol<StateData> {
  pub fn submit(mut self, userdata: &UserData) -> Res<Exasol<StateEnd>> {
    let userdata_skype = userdata.skype_as_str();
    let userdata_birthdate = &userdata.birth_date_as_string();
    let userdata_mailnum = userdata.emails_num();
    let userdata_mailnum_as_string = &userdata_mailnum.to_string();
    let userdata_addrnum = userdata.address_num();
    let userdata_addrnum_as_string = &userdata_addrnum.to_string();

    loop {
      let got_mailnum = self.state.got_mailnum();
      let got_addrnum = self.state.got_addrnum();
      let (command, mut args, state) = self.get_command_args_and_state()?;

      if command == "END" {
        self.stream.write_all(b"OK\n")?;
        self.stream.flush()?;

        info!("Successfully submitted data to server");
        return Ok(Exasol::make(self));
      }

      let arg1 = args.next().ok_or(Err::MissingArg)?;
      let hash = state.hash(arg1);

      let datum = match command {
        "NAME" => {
          debug!("Name:");
          userdata.name()
        }
        "SKYPE" => {
          debug!("Skype:");
          userdata_skype
        }
        "COUNTRY" => {
          debug!("Country:");
          userdata.country()
        }
        "BIRTHDATE" => {
          debug!("Birthdate:");
          userdata_birthdate
        }
        "MAILNUM" => {
          debug!("Mailnum:");
          self.state.set_got_mailnum();
          userdata_mailnum_as_string
        }
        "ADDRNUM" => {
          debug!("Addrnum:");
          self.state.set_got_addrnum();
          userdata_addrnum_as_string
        }
        command if command.starts_with("MAIL") => {
          debug!("MailN ({})", command);

          if !got_mailnum {
            return Err::no_mailnum();
          }

          let num = command[4..].parse::<usize>()?;

          if num < 1 || num > userdata_mailnum {
            return Err::invalid_mail_index(num);
          }

          userdata.email(num - 1)
        }
        command if command.starts_with("ADDRLINE") => {
          debug!("AddrN ({})", command);

          if !got_addrnum {
            return Err::no_addrnum();
          }

          let num = command[8..].parse::<usize>()?;

          if num < 1 || num > userdata_addrnum {
            return Err::invalid_address_index(num);
          }

          userdata.address_line(num - 1)
        }
        _ => {
          const EXPECTED: &str = "NAME|MAILNUM|MAILx|SKYPE|BIRTHDATE|COUNTRY|ADDRNUM|ADDRx";
          return Err::unknown_or_unexpected_command(command, EXPECTED);
        }
      };

      debug!("  Submitting `{}`", datum);

      self.stream.write_all(hex::encode(hash).as_bytes())?;
      self.stream.write_all(b" ")?;
      self.stream.write_all(datum.as_bytes())?;
      self.stream.write_all(b"\n")?;
      self.stream.flush()?;
    }
  }
}
