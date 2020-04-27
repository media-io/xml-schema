use crate::xsd::{list::List, restriction::Restriction, union::Union};
use heck::CamelCase;
use log::debug;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
  #[yaserde(attribute)]
  pub name: String,
  pub restriction: Option<Restriction>,
  pub list: Option<List>,
  pub union: Option<Union>,
}

impl SimpleType {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    _prefix: &Option<String>,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name.to_camel_case(), Span::call_site());

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #[yaserde(text)]
        pub content: String,
      }
    )
  }
}
