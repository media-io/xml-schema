use crate::xsd::element::Element;
use yaserde::YaDeserialize;
use std::io::prelude::*;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Sequence {
  #[yaserde(rename="element")]
  pub elements: Vec<Element>,
}
