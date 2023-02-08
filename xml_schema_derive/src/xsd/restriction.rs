use crate::xsd::{rust_types_mapping::RustTypesMapping, XsdContext};
use log::debug;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Restriction {
  #[yaserde(rename = "base", attribute)]
  pub base: Option<String>,
}

impl Restriction {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    if let Some(base) = &self.base {
      RustTypesMapping::get(context, base)
    } else {
      panic!("Missing base for restriction");
    }
  }
}
