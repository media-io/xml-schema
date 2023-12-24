use crate::xsd::{
  rust_types_mapping::RustTypesMapping, sequence::Sequence, Implementation, XsdContext,
};
use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Group {
  #[yaserde(attribute)]
  pub name: Option<String>,
  #[yaserde(attribute, rename = "ref")]
  pub reference: Option<String>,
  #[yaserde()]
  pub sequence: Option<Sequence>,
}

impl Implementation for Group {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.name.is_none() {
      return quote!();
    }
    let raw_name = self.name.clone().unwrap();

    let struct_name = Ident::new(&raw_name.to_upper_camel_case(), Span::call_site());

    let fields = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_field_implementation(context, prefix))
      .unwrap_or_default();

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }
    )
  }
}

impl Group {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    if let Some(reference) = &self.reference {
      RustTypesMapping::get(context, reference)
    } else {
      panic!("Missing reference for group");
    }
  }
}