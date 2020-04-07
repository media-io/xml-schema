use crate::xsd::{attribute::Attribute, rust_types_mapping::RustTypesMapping};
use log::debug;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root = "extension",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
  #[yaserde(attribute)]
  pub base: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
}

impl Extension {
  pub fn get_implementation(&self) -> TokenStream {
    let rust_type = RustTypesMapping::get(&self.base);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.get_implementation())
      .collect();

    let inner_attribute = if format!("{}", rust_type) == "String" {
      quote!(#[yaserde(text)])
    } else {
      quote!()
    };

    quote!(
      #inner_attribute
      pub content: #rust_type,
      #attributes
    )
  }
}
