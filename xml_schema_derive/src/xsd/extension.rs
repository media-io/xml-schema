use crate::xsd::{attribute::Attribute, rust_types_mapping::RustTypesMapping, sequence::Sequence};
use log::debug;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root = "extension",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
  #[yaserde(attribute)]
  pub base: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "sequence")]
  pub sequences: Vec<Sequence>,
}

impl Extension {
  pub fn get_implementation(&self) -> TokenStream {
    let rust_type = RustTypesMapping::get(&self.base);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.get_implementation())
      .collect();

    let inner_attribute = if format!("{}", rust_type) == "String" {
      quote!(#[yaserde(text)])
    } else {
      quote!()
    };

    quote!(
      #inner_attribute
      pub content: #rust_type,
      #attributes
    )
  }
}

#[test]
fn extension() {
  let st = Extension {
    base: "xs:string".to_string(),
    attributes: vec![],
    sequences: vec![],
  };

  let ts = st.get_implementation().to_string();
  assert!(ts == "# [ yaserde ( text ) ] pub content : String ,");
}

#[test]
fn extension_with_attributes() {
  use crate::xsd::attribute::Required;

  let st = Extension {
    base: "xs:string".to_string(),
    attributes: vec![
      Attribute {
        name: "attribute_1".to_string(),
        kind: "xs:string".to_string(),
        required: Required::Required,
      },
      Attribute {
        name: "attribute_2".to_string(),
        kind: "xs:bool".to_string(),
        required: Required::Optional,
      },
    ],
    sequences: vec![],
  };

  let ts = st.get_implementation().to_string();
  assert!(ts == "# [ yaserde ( text ) ] pub content : String , # [ yaserde ( attribute ) ] attribute_1 : String , # [ yaserde ( attribute ) ] attribute_2 : Option < Bool > ,");
}
