use crate::xsd::Implementation;
use proc_macro2::TokenStream;

use xml_schema::{ComplexContent, XsdContext};

impl Implementation for ComplexContent {
  fn get_field_implementation(&self, prefix: &Option<String>, context: &XsdContext) -> TokenStream {
    self
      .extension
      .as_ref()
      .unwrap()
      .get_field_implementation(prefix, context)
  }
}
