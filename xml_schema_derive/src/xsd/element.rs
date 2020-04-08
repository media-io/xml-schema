use crate::xsd::{
  complex_type::ComplexType, max_occurences::MaxOccurences, rust_types_mapping::RustTypesMapping,
};
use inflector::Inflector;
use log::info;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Element {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename = "type", attribute)]
  pub kind: String,
  #[yaserde(rename = "ref", attribute)]
  pub refers: String,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Vec<ComplexType>,
}

impl Element {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    _prefix: &Option<String>,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name, Span::call_site());
    let extern_type = self.get_identifier();

    let fields =
      if extern_type == "string" {
        quote!(
          #[yaserde(text)]
          pub content: String,
        )
      } else {
        quote!(
          #[yaserde(flatten)]
          pub content: #extern_type,
        )
      };

    quote! {
      #[derive(Clone, Debug, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }
    }
  }

  pub fn get_field_implementation(&self, prefix: &Option<String>) -> TokenStream {
    info!("Generate element {:?}", self.name.to_snake_case());
    if self.name.to_snake_case() == "" {
      return quote!();
    }
    let attribute_name = Ident::new(&self.name.to_snake_case(), Span::call_site());
    let yaserde_rename = &self.name;

    let rust_type = if self.complex_type.is_empty() {
      RustTypesMapping::get(&self.kind)
    } else if let Some(sequence) = &self.complex_type.first().unwrap().sequence {
      let element = sequence.elements.first().unwrap();
      let list_type = element.get_identifier();
      quote!(Vec<#list_type>)
    } else {
      unimplemented!()
    };

    let prefix_attribute = if let Some(prefix) = prefix {
      quote!(, prefix=#prefix)
    } else {
      quote!()
    };

    quote! {
      #[yaserde(rename=#yaserde_rename #prefix_attribute)]
      pub #attribute_name: #rust_type,
    }
  }

  pub fn get_identifier(&self) -> Ident {
    let complex_type_list: Vec<&str> = self.kind.split(':').collect();
    Ident::new(&complex_type_list.last().unwrap(), Span::call_site())
  }
}
