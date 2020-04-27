use crate::xsd::{sequence::Sequence, simple_content::SimpleContent};
use heck::CamelCase;
use log::{debug, info};
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct ComplexType {
  #[yaserde(attribute)]
  pub name: String,
  pub sequence: Option<Sequence>,
  #[yaserde(rename = "simpleContent")]
  pub simple_content: Option<SimpleContent>,
}

impl ComplexType {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name.to_camel_case(), Span::call_site());

    info!("Generate sequence");
    let sequence = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_implementation(prefix))
      .unwrap_or_else(|| quote!());

    info!("Generate simple content");
    let simple_content = self
      .simple_content
      .as_ref()
      .map(|simple_content| simple_content.get_implementation())
      .unwrap_or_else(|| quote!());

    let namespace_definition = if self.name == "AssetType" {
      quote!(#[yaserde(root= "Asset", prefix = "am", namespace="am: http://www.smpte-ra.org/schemas/429-9/2007/AM")])
    } else {
      namespace_definition.clone()
    };

    let sub_types_implementation = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_sub_types_implementation(&namespace_definition, prefix))
      .unwrap_or_else(|| quote!());

    quote! {
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #sequence
        #simple_content
      }

      #sub_types_implementation
    }
  }

  pub fn get_field_implementation(&self, prefix: &Option<String>) -> TokenStream {
    self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_field_implementation(prefix))
      .unwrap_or_else(|| quote!())
  }
}
