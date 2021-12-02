#![warn(clippy::all)]

use crate::Res;
use log::{debug, error};
use openssl::pkey::PKey;
use openssl::ssl::{Ssl, SslContext, SslContextBuilder, SslMethod, SslStream};
use openssl::x509::X509;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn create_ssl_context(cert_file: &Path, keylog_file: Option<&Path>) -> Res<SslContext> {
  debug!("Reading certificate chain and key from {}", cert_file.display());

  let cert_file = File::open(cert_file)?;
  let mut reader = BufReader::new(cert_file);
  let mut contents = vec![];
  let bytes = reader.read_to_end(&mut contents)?;
  debug!("Read certificate chain and key file: {} bytes", bytes);

  let pkey = PKey::private_key_from_pem(&contents)?;
  debug!("Parsed private key: {:?}", pkey);

  let cert = X509::from_pem(&contents)?;
  debug!("Parsed certificate: {:?}", cert);

  let mut ctx_builder = SslContextBuilder::new(SslMethod::tls())?;
  ctx_builder.set_private_key(&pkey)?;
  ctx_builder.set_certificate(&cert)?;
  ctx_builder.check_private_key()?;

  if let Some(keylog_file) = keylog_file {
    let keylog_filename = PathBuf::from(keylog_file);
    let keylog_file = Arc::new(Mutex::new(File::create(keylog_file)?));
    debug!("Created keylog file `{}`", keylog_filename.display());

    ctx_builder.set_keylog_callback(move |_ssl, msg| match keylog_file.lock() {
      Ok(mut keylog_file) => {
        if let Err(e) = writeln!(keylog_file, "{}", msg) {
          error!("Error writing to keylog file `{}`: {}", keylog_filename.display(), e);
        }

        if let Err(e) = keylog_file.flush() {
          error!("Error flushing keylog file `{}`: {}", keylog_filename.display(), e);
        }
      }
      Err(e) => error!("Error accessing keylog file `{}`: {}", keylog_filename.display(), e),
    });
  }

  let ctx = ctx_builder.build();
  debug!("Created SSL context");
  Ok(ctx)
}

pub(crate) fn create_ssl_stream(
  cert_file: &Path,
  keylog_file: Option<&Path>,
  address: &str,
) -> Res<SslStream<TcpStream>> {
  let ctx = create_ssl_context(cert_file, keylog_file)?;
  let ssl = Ssl::new(&ctx)?;
  let stream = TcpStream::connect(address)?;
  debug!("Connected TCP stream: {:?}", stream);
  let ssl_stream = SslStream::new(ssl, stream)?;
  debug!("Created SSL stream: {:?}", ssl_stream);
  Ok(ssl_stream)
}
