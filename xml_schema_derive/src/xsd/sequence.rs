use crate::xsd::element::Element;
use log::info;
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
    let code = self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(prefix))
      .collect();

    // println!("{}", code);
    code
  }
}
