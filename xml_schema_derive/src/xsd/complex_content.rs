use crate::xsd::{extension::Extension, xsd_context::XsdContext};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct ComplexContent {
  pub extension: Option<Extension>,
}

impl ComplexContent {
  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    self
      .extension
      .as_ref()
      .unwrap()
      .get_field_implementation(context, prefix)
  }
}
