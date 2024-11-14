use crate::xsd::{Implementation, XsdContext};
use proc_macro2::TokenStream;

use xml_schema::SimpleContent;

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

  fn get_field_implementation(&self, context: &XsdContext, prefix: &Option<String>) -> TokenStream {
    self.extension.get_field_implementation(context, prefix)
  }
}
