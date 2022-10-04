use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use diesel::prelude::{Queryable};

use super::generator;

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

#[derive(Serialize, Debug, Queryable)]
pub struct PasswordMeta {
  id: i32,
  account_name: String,
  secret: String,
  institution: String,
  industry: Industry,
  is_legacy: bool,
  is_work: bool,
  date_created: DateTime<Utc>
}

impl PasswordMeta {
  fn gen_pw(&self) -> String {
    generator::gen_pw(&self.institution, &self.industry, &self.secret, &self.account_name)
  }

  fn gen_legacy_pw(&self) -> String {
    generator::gen_legacy_pw(&self.institution, &self.industry, &self.secret)
  }
}
