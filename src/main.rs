#![warn(clippy::all)]

use exasol::protocol::Exasol;
use exasol::userdata::UserData;
use exasol::Res;
use humantime::format_duration as humantime;
use log::{debug, error, info, trace, warn};
use rand::distributions::DistIter;
use rand::prelude::StdRng;
use rand::{distributions, Rng, SeedableRng};
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Exasol coding challenge")]
#[structopt(author = "Fred Morcos <fm@fredmorcos.com>")]
struct Opt {
  /// Verbose output (can be specified multiple times)
  #[structopt(short, long, parse(from_occurrences))]
  verbose: u8,

  /// File to import user data used for submisison.
  #[structopt(short, long, name = "DATA-FILE")]
  data_file: PathBuf,

  /// Certificate file.
  #[structopt(short, long, name = "CERT-FILE")]
  cert_file: PathBuf,

  /// Keylog file (e.g. for use with Wireshark).
  #[structopt(short, long, name = "KEYLOG-FILE")]
  keylog_file: Option<PathBuf>,
}

fn run(opt: &Opt) -> Res<()> {
  let user_data = UserData::new(&opt.data_file)?;

  let protocol = Exasol::new(&opt.cert_file, opt.keylog_file.as_deref(), "18.202.148.130:3336")?;
  let protocol = protocol.connect()?.handshake()?.pow()?;

  Ok(())
}

fn main() {
  let start_time = Instant::now();
  let opt = Opt::from_args();

  let log_level = match opt.verbose {
    0 => log::LevelFilter::Off,
    1 => log::LevelFilter::Error,
    2 => log::LevelFilter::Warn,
    3 => log::LevelFilter::Info,
    4 => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };

  let logger = env_logger::Builder::new().filter_level(log_level).try_init();
  let have_logger = if let Err(e) = logger {
    eprintln!("Error initializing logger: {}", e);
    false
  } else {
    true
  };

  error!("Error output enabled.");
  warn!("Warning output enabled.");
  info!("Info output enabled.");
  debug!("Debug output enabled.");
  trace!("Trace output enabled.");

  if let Err(e) = run(&opt) {
    if have_logger {
      error!("Error: {}", e);
    } else {
      eprintln!("Error: {}", e);
    }
  }

  if have_logger {
    info!("Total time: {}", humantime(Instant::now().duration_since(start_time)));
  } else {
    eprintln!("Total time: {}", humantime(Instant::now().duration_since(start_time)));
  }
}
