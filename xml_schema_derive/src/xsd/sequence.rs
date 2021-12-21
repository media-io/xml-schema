use crate::xsd::{choice::Choice, element::Element, Implementation, XsdContext};
use log::{debug, info};
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Sequence {
  #[yaserde(rename = "element")]
  pub elements: Vec<Element>,
  #[yaserde(rename = "choice")]
  pub choices: Vec<Choice>,
}

impl Implementation for Sequence {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    info!("Generate elements");
    let elements: TokenStream = self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(context, prefix, false, false))
      .collect();

    let choices: TokenStream = self
      .choices
      .iter()
      .map(|choice| choice.get_field_implementation(context, prefix))
      .collect();

    quote!(
      #elements
      #choices
    )
  }
}

impl Sequence {
  pub fn get_sub_types_implementation(
    &self,
    context: &XsdContext,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    info!("Generate sub types implementation");
    self
      .elements
      .iter()
      .map(|element| element.get_subtypes_implementation(namespace_definition, prefix, context))
      .collect()
  }

  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let elements: TokenStream = self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(context, prefix, false, false))
      .collect();

    let choices: TokenStream = self
      .choices
      .iter()
      .map(|choice| choice.get_field_implementation(context, prefix))
      .collect();

    quote!(
      #elements
      #choices
    )
  }
}
