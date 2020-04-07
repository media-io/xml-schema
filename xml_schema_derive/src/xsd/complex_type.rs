use crate::xsd::sequence::Sequence;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct ComplexType {
  #[yaserde(attribute)]
  pub name: String,
  pub sequence: Sequence,
}

impl ComplexType {
  pub fn get_implementation(&self) -> TokenStream {
    let struct_name = Ident::new(&self.name, Span::call_site());

    let elements: TokenStream = 
      self.sequence.elements.iter().map(|sub_element| {
        sub_element.get_field_implementation()
        })
      .collect();

    let namespace = "http://www.smpte-ra.org/schemas/429-9/2007/AM";
    let prefix = "am";
    let namespace_attribute = format!("{}: {}", prefix, namespace);

    quote! {
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #[yaserde(prefix=#prefix, namespace=#namespace_attribute)]
      pub struct #struct_name {
        #elements
      }
    }
  }
}