use crate::xsd::rust_types_mapping::RustTypesMapping;
use log::debug;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Attribute {
  #[yaserde(prefix = "xs", attribute)]
  pub name: String,
  #[yaserde(rename = "type", attribute)]
  pub kind: String,
  #[yaserde(rename = "use", attribute)]
  pub required: Required,
}

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Required {
  #[yaserde(rename = "optional")]
  Optional,
  #[yaserde(rename = "required")]
  Required,
}

impl Default for Required {
  fn default() -> Self {
    Required::Optional
  }
}

impl Attribute {
  pub fn get_implementation(&self) -> TokenStream {
    let name =
      if self.name == "type" {
        "kind".to_string()
      } else {
        self.name.clone()
      };

    let field_name = Ident::new(&name, Span::call_site());
    let rust_type = RustTypesMapping::get(&self.kind);

    let rust_type = if self.required == Required::Optional {
      quote!(Option<#rust_type>)
    } else {
      quote!(#rust_type)
    };

    quote!(
      #[yaserde(attribute)]
      #field_name: #rust_type,
    )
  }
}
