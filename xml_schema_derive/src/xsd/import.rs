use log::debug;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Import {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(attribute)]
  pub namespace: Option<String>,
  #[yaserde(rename = "schemaLocation", attribute)]
  pub schema_location: Option<String>,
}
