use crate::xsd::Implementation;
use proc_macro2::TokenStream;

use xml_schema::{ComplexContent, XsdContext};

impl Implementation for ComplexContent {
  fn get_field_implementation(&self, context: &XsdContext, prefix: &Option<String>) -> TokenStream {
    self
      .extension
      .as_ref()
      .unwrap()
      .get_field_implementation(context, prefix)
  }
}
