use crate::xsd::element::Element;
use log::{debug, info};
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Sequence {
  #[yaserde(rename = "element")]
  pub elements: Vec<Element>,
}

impl Sequence {
  pub fn get_implementation(&self, prefix: &Option<String>) -> TokenStream {
    info!("Generate elements");
    self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(prefix, false))
      .collect()
  }

  pub fn get_sub_types_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    info!("Generate sub types implementation");
    self
      .elements
      .iter()
      .map(|element| element.get_subtypes_implementation(namespace_definition, prefix))
      .collect()
  }

  pub fn get_field_implementation(&self, prefix: &Option<String>) -> TokenStream {
    self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(prefix, true))
      .collect()
  }
}
