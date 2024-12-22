use crate::xsd::{Implementation, XsdContext};
use proc_macro2::TokenStream;

use xml_schema::Annotation;

impl Implementation for Annotation {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
  ) -> TokenStream {
    log::info!("Generate annotation");

    let documentation = self
      .documentation
      .iter()
      .map(|documentation| quote!(#[doc = #documentation]));

    quote!(#(#documentation)*)
  }
}
