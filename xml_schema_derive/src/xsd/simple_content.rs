use crate::xsd::extension::Extension;
use log::debug;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleContent {
  #[yaserde(prefix = "xs", rename = "extension")]
  pub extension: Extension,
}

impl SimpleContent {
  pub fn get_implementation(&self) -> TokenStream {
    self.extension.get_implementation()
  }
}
