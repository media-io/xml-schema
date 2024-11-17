use crate::xsd::{Implementation, XsdContext};
use log::info;
use proc_macro2::TokenStream;

use xml_schema::Sequence;

impl Implementation for Sequence {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    info!("Generate elements");
    self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(prefix, context))
      .collect()
  }

  fn get_sub_types_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    info!("Generate sub types implementation");
    self
      .elements
      .iter()
      .map(|element| element.get_subtypes_implementation(namespace_definition, prefix, context))
      .collect()
  }

  fn get_field_implementation(&self, prefix: &Option<String>, context: &XsdContext) -> TokenStream {
    self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(prefix, context))
      .collect()
  }
}
