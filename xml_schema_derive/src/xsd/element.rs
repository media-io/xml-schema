use crate::xsd::{
  complex_type::ComplexType, max_occurences::MaxOccurences, rust_types_mapping::RustTypesMapping,
};
use heck::{CamelCase, SnakeCase};
use log::{debug, info};
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
    prefix: &Option<String>,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name, Span::call_site());

    let fields = match self.get_identifier().as_ref() {
      "string" => quote!(
        #[yaserde(text)]
        pub content: String,
      ),
      "" => self
        .complex_type
        .iter()
        .map(|complex_type| complex_type.get_field_implementation(prefix))
        .collect(),
      _ => {
        let extern_type = self.get_ident_identifier();
        quote!(
          #[yaserde(flatten)]
          pub content: #extern_type,
        )
      }
    };

    quote! {
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }
    }
  }

  pub fn get_subtypes_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    if self.complex_type.is_empty() {
      return quote!();
    }

    self.get_implementation(namespace_definition, prefix)
  }

  pub fn get_field_implementation(&self, prefix: &Option<String>, multiple: bool) -> TokenStream {
    if self.name == "" {
      return quote!();
    }

    if self.kind == "md:CompObjEntry-type" {
      return quote!();
    }

    let name =
      if self.name.to_lowercase() == "type" {
        "Kind".to_string()
      } else {
        self.name.to_snake_case().clone()
      };

    info!("Generate element {:?}", name);

    let name = if multiple {
      format!("{}s", name)
    } else {
      name
    };

    let attribute_name = Ident::new(&name, Span::call_site());
    let yaserde_rename = &self.name;

    let rust_type = if self.complex_type.is_empty() {
      RustTypesMapping::get(&self.kind)
    } else if self.complex_type.first().unwrap().sequence.is_some() {
      let list_wrapper = Ident::new(&self.name, Span::call_site());
      quote!(#list_wrapper)
    } else if self.complex_type.first().unwrap().simple_content.is_some() {
      quote!(String)
    } else {
      println!("UNIMPLEMENTED {:?}", self);
      unimplemented!()
    };

    let rust_type = if multiple {
      quote!(Vec<#rust_type>)
    } else {
      rust_type
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

  pub fn get_identifier(&self) -> String {
    let complex_type_list: Vec<String> = self.kind.split(':').map(|e| e.to_string()).collect();
    complex_type_list.last().unwrap().to_owned()
  }

  pub fn get_ident_identifier(&self) -> Ident {
    let identifier = self.get_identifier().to_camel_case();
    Ident::new(&identifier, Span::call_site())
  }
}
