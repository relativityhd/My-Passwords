use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::generator;

// TODO: Add Gov
#[derive(Serialize, Deserialize, Debug)]
pub enum Industry {
  Tech,     // -> @
  Games,    // -> !
  Social,   // -> #
  Finance,  // -> $
  Shopping, // -> *
  Science,  // -> ?
  Other     // -> &
}

#[derive(Serialize, Debug)]
pub struct Account {
  id: i32,
  account_name: String, // Users username
  secret: String, // Users secret
  institution: String,
  industry: Industry,
  is_legacy: bool,
  is_work: bool,
  date_created: DateTime<Utc>
}

impl Account {
  fn gen_pw(&self) -> String {
    generator::gen_pw(&self.institution, &self.industry, &self.secret, &self.account_name)
  }

  fn gen_legacy_pw(&self) -> String {
    generator::gen_legacy_pw(&self.institution, &self.industry, &self.secret)
  }
}
