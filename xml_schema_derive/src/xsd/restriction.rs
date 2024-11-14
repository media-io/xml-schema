use crate::xsd::{rust_types_mapping::RustTypesMapping, XsdContext};
use proc_macro2::TokenStream;

use crate::xsd::Implementation;
use xml_schema::Restriction;

impl Implementation for Restriction {
  fn get_type_implementation(&self, context: &XsdContext, _prefix: &Option<String>) -> TokenStream {
    if let Some(base) = &self.base {
      RustTypesMapping::get(context, base)
    } else {
      panic!("Missing base for restriction");
    }
  }
}
