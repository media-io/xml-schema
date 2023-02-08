use crate::xsd::{extension::Extension, Implementation, XsdContext};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleContent {
  #[yaserde(prefix = "xs", rename = "extension")]
  pub extension: Extension,
}

impl Implementation for SimpleContent {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    self
      .extension
      .implement(namespace_definition, prefix, context)
  }
}

impl SimpleContent {
  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    self.extension.get_field_implementation(context, prefix)
  }
}
