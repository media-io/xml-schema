//! The children of a choice are mapped to Option fields.
//! Generating an enum would have been the better way but the choice element
//! may not have a name, so it's impossible to name the generated Rust enum.
//! The enum would have been nice to avoid runtime checks that only a single choice element is used.

use crate::xsd::{
  annotation::Annotation, attribute::Attribute, element::Element, max_occurences::MaxOccurences,
  Implementation, XsdContext,
};
use log::{debug, info};
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "choice"
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Choice {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "element")]
  pub element: Vec<Element>,
}

impl Implementation for Choice {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let elements: TokenStream = self
      .element
      .iter()
      .map(|element| element.implement(&namespace_definition, prefix, context))
      .collect();

    quote! {
      #elements
    }
  }
}

impl Choice {
  pub fn get_sub_types_implementation(
    &self,
    context: &XsdContext,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    info!("Generate choice sub types implementation");
    self
      .element
      .iter()
      .map(|element| element.get_subtypes_implementation(namespace_definition, prefix, context))
      .collect()
  }

  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    info!("Generate choice elements");
    self
      .element
      .iter()
      .map(|element| element.get_field_implementation(context, prefix, false, true))
      .collect()
  }
}
