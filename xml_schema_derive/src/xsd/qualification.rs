use log::debug;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Qualification {
  #[yaserde(rename = "qualified")]
  Qualidified,
  #[yaserde(rename = "unqualified")]
  Unqualified,
}

impl Default for Qualification {
  fn default() -> Self {
    Qualification::Unqualified
  }
}
