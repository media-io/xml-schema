use crate::xsd::{
  max_occurences::MaxOccurences,
  sequence::Sequence,
};
use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use yaserde::YaDeserialize;
use std::io::prelude::*;
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Element {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename="type", attribute)]
  pub kind: String,
  #[yaserde(rename="minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename="maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename="complexType")]
  pub complex_type: Vec<Sequence>,
}

impl Element {
  pub fn get_implementation(&self) -> TokenStream {
    let struct_name = Ident::new(&self.name, Span::call_site());
    let complex_type_list: Vec<&str> = self.kind.split(":").collect();
    let extern_type = Ident::new(&complex_type_list.last().unwrap(), Span::call_site());

    let namespace = "http://www.smpte-ra.org/schemas/429-9/2007/AM";
    let prefix = "am";
    let namespace_attribute = format!("{}: {}", prefix, namespace);

    quote! {
      #[derive(Clone, Debug, PartialEq, YaDeserialize, YaSerialize)]
      #[yaserde(prefix=#prefix, namespace=#namespace_attribute)]
      pub struct #struct_name {
        #[yaserde(flatten)]
        pub content: #extern_type,
      }
    }
  }

  pub fn get_field_implementation(&self) -> TokenStream {
    let attribute_name = Ident::new(&self.name.to_snake_case(), Span::call_site());
    let yaserde_rename = &self.name;
    let rust_type = self.get_rust_type();

    // let namespace = "http://www.smpte-ra.org/schemas/429-9/2007/AM";
    let prefix = "am";
    // let namespace_attribute = format!("{}: {}", prefix, namespace);

    quote!{
      #[yaserde(rename=#yaserde_rename, prefix=#prefix)]
      pub #attribute_name: #rust_type,
    }
  }

  fn get_rust_type(&self) -> TokenStream {
    match self.kind.as_str() {
      "xs:boolean" => quote!(bool),
      "xs:positiveInteger" => quote!(u32),
      "xs:byte" => quote!(i8),
      "xs:unsignedByte" => quote!(u8),
      "xs:short" => quote!(i16),
      "xs:unsignedShort" => quote!(u16),
      "xs:int" => quote!(i32),
      "xs:integer" => quote!(i32),
      "xs:unsignedInt" => quote!(u32),
      "xs:long" => quote!(i64),
      "xs:unsignedLong" => quote!(u64),

      "xs:string" => quote!(String),
      "xs:normalizedString" => quote!(String),
      "xs:anyURI" => quote!(String),
      "xs:token" => quote!(String),
      // "xs:hexBinary" => quote!(String),
      // "xs:dateTime" => quote!(),
      // "dcml:UUIDType" => quote!(uuid::Uuid),
      "dcml:UserTextType" => quote!(String),
      _ => quote!(String),
    }
  }
}