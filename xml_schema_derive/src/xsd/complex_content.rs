use crate::xsd::extension::Extension;
use log::debug;
use std::io::Read;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct ComplexContent {
  pub extension: Option<Extension>,
}
