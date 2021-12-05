#![warn(clippy::all)]

use crate::Res;
use chrono::NaiveDate;
use log::debug;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct UserData {
  name: String,
  emails: Vec<String>,
  skype: Option<String>,
  // Day, Month, Year and must be formatted to %d.%m.%Y
  #[serde(deserialize_with = "deserialize_birthdate")]
  birth_date: NaiveDate,
  // https://www.countries-ofthe-world.com/all-countries.html
  country: String,
  address: Vec<String>,
}

fn deserialize_birthdate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  NaiveDate::parse_from_str(&s, "%d.%m.%Y").map_err(serde::de::Error::custom)
}

impl UserData {
  pub fn new(filename: &Path) -> Res<Self> {
    debug!("Reading user data from {}:", filename.display());

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let user_data: Self = serde_json::from_reader(reader).map_err(Box::new)?;

    debug!("  Name: {}", user_data.name);

    debug!("  Emails:");
    for email in &user_data.emails {
      debug!("    {}", email);
    }

    debug!("  Skype: {:?}", user_data.skype);
    debug!("  Birthdate: {:?}", user_data.birth_date);
    debug!("  Country: {}", user_data.country);

    debug!("  Address:");
    for address_line in &user_data.address {
      debug!("    {}", address_line);
    }

    Ok(user_data)
  }

  /// Get a reference to the user's name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Get the number of email addresses the user has.
  pub fn emails_num(&self) -> usize {
    self.emails.len()
  }

  /// Get a reference to the user's nth email address.
  pub fn email(&self, n: usize) -> &str {
    &self.emails[n]
  }

  /// Get a reference to the user's skype.
  pub fn skype(&self) -> Option<&String> {
    self.skype.as_ref()
  }

  /// Get the user's skype address as a string.
  pub fn skype_as_str(&self) -> &str {
    self.skype.as_ref().map(|v| v.as_ref()).unwrap_or("N/A")
  }

  /// Get a reference to the user's birth date.
  pub fn birth_date(&self) -> &NaiveDate {
    &self.birth_date
  }

  /// Get the user's birth date as string.
  pub fn birth_date_as_string(&self) -> String {
    self.birth_date.format("%d.%m.%Y").to_string()
  }

  /// Get a reference to the user's country.
  pub fn country(&self) -> &str {
    &self.country
  }

  /// Get the number of address lines the user has.
  pub fn address_num(&self) -> usize {
    self.address.len()
  }

  /// Get a reference to the user's nth address line.
  pub fn address_line(&self, n: usize) -> &str {
    &self.address[n]
  }
}
