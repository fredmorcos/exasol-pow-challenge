#![warn(clippy::all)]

use derive_more::Display;
use exasol::ssl::create_ssl_stream;
use exasol::userdata::UserData;
use exasol::Res;
use humantime::format_duration as humantime;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, Display)]
#[display(fmt = "{}")]
enum ExasolErr {
  #[display(fmt = "Certificate file `{}` not found", _0)]
  CertFile(String),
}

impl ExasolErr {
  fn cert_file<T>(filename: String) -> Res<T> {
    Err(Box::new(ExasolErr::CertFile(filename)))
  }
}

impl Error for ExasolErr {}

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
  let mut ssl_stream = create_ssl_stream(&opt.cert_file, "18.202.148.130:3336")?;

  debug!("Connecting...");
  ssl_stream.connect()?;

  let mut buffer = vec![];
  loop {
    loop {
      let bytes = ssl_stream.read_to_end(&mut buffer)?;
      debug!("Received {} bytes: {:?}", bytes, buffer);

      if bytes == 0 {
        std::thread::sleep(std::time::Duration::from_millis(500));
      } else {
        break;
      }
    }

    let args = std::str::from_utf8(&buffer)?.trim();
    debug!("  As string: `{}`", args);

    let mut args = args.trim().split(' ');
    match args.next() {
      Some(command) => match command {
        "HELO" => {
          debug!("Responding to HELO");
          let _ = ssl_stream.write_all("HELO\n".as_bytes())?;
          let _ = ssl_stream.flush()?;
        }
        _ => {
          error!("Unknown command from server: `{}`", command);
          break;
        }
      },
      None => {
        error!("Empty response from server");
        break;
      }
    }

    buffer.clear();
  }

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
