use crate::xsd::{rust_types_mapping::RustTypesMapping, XsdContext};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Group {
  #[yaserde(attribute)]
  pub r#ref: String,
}

impl Group {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    RustTypesMapping::get(context, &self.r#ref)
  }
}
