
use yaserde::YaDeserialize;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Qualification {
  #[yaserde(rename="qualified")]
  Qualidified,
  #[yaserde(rename="unqualified")]
  Unqualified,
}

impl Default for Qualification {
  fn default() -> Self {
    Qualification::Unqualified
  }
}
