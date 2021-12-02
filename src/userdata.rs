#![warn(clippy::all)]

use crate::Res;
use derive_more::Display;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Display, Clone, Copy)]
#[display(fmt = "{}")]
pub enum Country {
  Afghanistan,
  Albania,
  Algeria,
  Andorra,
  Angola,
  #[serde(rename = "Antigua and Barbuda")]
  #[display(fmt = "Antigua and Barbuda")]
  AntiguaAndBarbuda,
  Argentina,
  Armenia,
  Australia,
  Austria,
  Azerbaijan,
  Bahamas,
  Bahrain,
  Bangladesh,
  Barbados,
  Belarus,
  Belgium,
  Belize,
  Benin,
  Bhutan,
  Bolivia,
  #[serde(rename = "Bosnia and Herzegovina")]
  #[display(fmt = "Bosnia and Herzegovina")]
  BosniaAndHerzegovina,
  Botswana,
  Brazil,
  Brunei,
  Bulgaria,
  #[serde(rename = "Burkina Faso")]
  #[display(fmt = "Burkina Faso")]
  BurkinaFaso,
  Burundi,
  #[serde(rename = "Cabo Verde")]
  #[display(fmt = "Cabo Verde")]
  CaboVerde,
  Cambodia,
  Cameroon,
  Canada,
  #[serde(rename = "Central African Republic")]
  #[display(fmt = "Central African Republic")]
  CentralAfricanRepublic,
  Chad,
  Chile,
  China,
  Colombia,
  Comoros,
  #[serde(rename = "Democratic Republic of the Congo")]
  #[display(fmt = "Democratic Republic of the Congo")]
  DemocraticRepublicOfTheCongo,
  #[serde(rename = "Republic of the Congo")]
  #[display(fmt = "Republic of the Congo")]
  RepublicOfTheCongo,
  #[serde(rename = "Costa Rica")]
  #[display(fmt = "Costa Rica")]
  CostaRica,
  #[serde(rename = "Cote d'Ivoire")]
  #[display(fmt = "Cote d'Ivoire")]
  CoteDIvoire,
  Croatia,
  Cuba,
  Cyprus,
  Czechia,
  Denmark,
  Djibouti,
  Dominica,
  #[serde(rename = "Dominican Republic")]
  #[display(fmt = "Dominican Republic")]
  DominicanRepublic,
  Ecuador,
  Egypt,
  #[serde(rename = "El Salvador")]
  #[display(fmt = "El Salvador")]
  ElSalvador,
  #[serde(rename = "Equatorial Guinea")]
  #[display(fmt = "Equatorial Guinea")]
  EquatorialGuinea,
  Eritrea,
  Estonia,
  Eswatini,
  Ethiopia,
  Fiji,
  Finland,
  France,
  Gabon,
  Gambia,
  Georgia,
  Germany,
  Ghana,
  Greece,
  Grenada,
  Guatemala,
  Guinea,
  #[serde(rename = "Guinea-Bissau")]
  #[display(fmt = "Guinea-Bissau")]
  GuineaBissau,
  Guyana,
  Haiti,
  Honduras,
  Hungary,
  Iceland,
  India,
  Indonesia,
  Iran,
  Iraq,
  Ireland,
  Israel,
  Italy,
  Jamaica,
  Japan,
  Jordan,
  Kazakhstan,
  Kenya,
  Kiribati,
  Kosovo,
  Kuwait,
  Kyrgyzstan,
  Laos,
  Latvia,
  Lebanon,
  Lesotho,
  Liberia,
  Libya,
  Liechtenstein,
  Lithuania,
  Luxembourg,
  Madagascar,
  Malawi,
  Malaysia,
  Maldives,
  Mali,
  Malta,
  #[serde(rename = "Marshall Islands")]
  #[display(fmt = "Marshall Islands")]
  MarshallIslands,
  Mauritania,
  Mauritius,
  Mexico,
  Micronesia,
  Moldova,
  Monaco,
  Mongolia,
  Montenegro,
  Morocco,
  Mozambique,
  Myanmar,
  Namibia,
  Nauru,
  Nepal,
  Netherlands,
  #[serde(rename = "New Zealand")]
  #[display(fmt = "New Zealand")]
  NewZealand,
  Nicaragua,
  Niger,
  Nigeria,
  #[serde(rename = "North Korea")]
  #[display(fmt = "North Korea")]
  NorthKorea,
  #[serde(rename = "North Macedonia")]
  #[display(fmt = "North Macedonia")]
  NorthMacedonia,
  Norway,
  Oman,
  Pakistan,
  Palau,
  Palestine,
  Panama,
  #[serde(rename = "Papua New Guinea")]
  #[display(fmt = "Papua New Guinea")]
  PapuaNewGuinea,
  Paraguay,
  Peru,
  Philippines,
  Poland,
  Portugal,
  Qatar,
  Romania,
  Russia,
  Rwanda,
  #[serde(rename = "Saint Kitts and Nevis")]
  #[display(fmt = "Saint Kitts and Nevis")]
  SaintKittsAndNevis,
  #[serde(rename = "Saint Lucia")]
  #[display(fmt = "Saint Lucia")]
  SaintLucia,
  #[serde(rename = "Saint Vincent and the Grenadines")]
  #[display(fmt = "Saint Vincent and the Grenadines")]
  SaintVincentAndTheGrenadines,
  Samoa,
  #[serde(rename = "San Marino")]
  #[display(fmt = "San Marino")]
  SanMarino,
  #[serde(rename = "Sao Tome and Principe")]
  #[display(fmt = "Sao Tome and Principe")]
  SaoTomeAndPrincipe,
  #[serde(rename = "Saudi Arabia")]
  #[display(fmt = "Saudi Arabia")]
  SaudiArabia,
  Senegal,
  Serbia,
  Seychelles,
  #[serde(rename = "Sierra Leone")]
  #[display(fmt = "Sierra Leone")]
  SierraLeone,
  Singapore,
  Slovakia,
  Slovenia,
  #[serde(rename = "Solomon Islands")]
  #[display(fmt = "Solomon Islands")]
  SolomonIslands,
  Somalia,
  #[serde(rename = "South Africa")]
  #[display(fmt = "South Africa")]
  SouthAfrica,
  #[serde(rename = "South Korea")]
  #[display(fmt = "South Korea")]
  SouthKorea,
  #[serde(rename = "South Sudan")]
  #[display(fmt = "South Sudan")]
  SouthSudan,
  Spain,
  #[serde(rename = "Sri Lanka")]
  #[display(fmt = "Sri Lanka")]
  SriLanka,
  Sudan,
  Suriname,
  Sweden,
  Switzerland,
  Syria,
  Taiwan,
  Tajikistan,
  Tanzania,
  Thailand,
  #[serde(rename = "Timor-Leste")]
  #[display(fmt = "Timor-Leste")]
  TimorLeste,
  Togo,
  Tonga,
  #[serde(rename = "Trinidad and Tobago")]
  #[display(fmt = "Trinidad and Tobago")]
  TrinidadAndTobago,
  Tunisia,
  Turkey,
  Turkmenistan,
  Tuvalu,
  Uganda,
  Ukraine,
  #[serde(rename = "United Arab Emirates (UAE)")]
  #[display(fmt = "United Arab Emirates (UAE)")]
  UnitedArabEmirates,
  #[serde(rename = "United Kingdom (UK)")]
  #[display(fmt = "United Kingdom (UK)")]
  UnitedKingdom,
  #[serde(rename = "United States of America (USA)")]
  #[display(fmt = "United States of America (USA)")]
  UnitedStatesOfAmerica,
  Uruguay,
  Uzbekistan,
  Vanuatu,
  #[serde(rename = "Vatican City (Holy See)")]
  #[display(fmt = "Vatican City (Holy See)")]
  VaticanCityHolySee,
  Venezuela,
  Vietnam,
  Yemen,
  Zambia,
  Zimbabwe,
}

#[derive(Deserialize)]
pub struct UserData {
  name: String,
  emails: Vec<String>,
  skype: Option<String>,
  // Day, Month, Year and must be formatted to %d.%m.%Y
  birth_date: (u8, u8, u16),
  // https://www.countries-ofthe-world.com/all-countries.html
  country: Country,
  address: Vec<String>,
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

  /// Get a reference to the user's emails.
  pub fn emails(&self) -> &[String] {
    &self.emails
  }

  /// Get a reference to the user's skype.
  pub fn skype(&self) -> Option<&String> {
    self.skype.as_ref()
  }

  /// Get a reference to the user's birth date.
  pub fn birth_date(&self) -> (u8, u8, u16) {
    self.birth_date
  }

  /// Get a reference to the user data's country.
  pub fn country(&self) -> Country {
    self.country
  }

  /// Get a reference to the user's address.
  pub fn address(&self) -> &[String] {
    &self.address
  }
}
