use crate::xsd::{rust_types_mapping::RustTypesMapping, simple_type::SimpleType, XsdContext};
use log::debug;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attribute",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Attribute {
  #[yaserde(prefix = "xs", attribute)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute)]
  pub kind: Option<String>,
  // #[yaserde(attribute)]
  // pub default: Option<String>,
  // #[yaserde(attribute)]
  // pub fixed: Option<String>,
  #[yaserde(rename = "use", attribute)]
  pub required: Required,
  #[yaserde(rename = "ref", attribute)]
  pub reference: Option<String>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
}

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Required {
  #[yaserde(rename = "optional")]
  Optional,
  #[yaserde(rename = "required")]
  Required,
}

impl Default for Required {
  fn default() -> Self {
    Required::Optional
  }
}

impl Attribute {
  pub fn get_implementation(&self, context: &XsdContext) -> TokenStream {
    if self.name.is_none() {
      return quote!();
    }
    let name = self.name.clone().unwrap();

    let name = if name == "type" {
      "kind".to_string()
    } else {
      name
    };

    let field_name = Ident::new(&name, Span::call_site());

    let rust_type = match (self.reference.as_ref(), self.kind.as_ref()) {
      (None, Some(kind)) => RustTypesMapping::get(context, &kind),
      (Some(reference), None) => RustTypesMapping::get(context, &reference),
      (_, _) => panic!("Not implemented Rust type for: {:?}", self),
    };

    let rust_type = if self.required == Required::Optional {
      quote!(Option<#rust_type>)
    } else {
      quote!(#rust_type)
    };

    quote!(
      #[yaserde(attribute)]
      pub #field_name: #rust_type,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn string_attribute() {
    let attribute = Attribute {
      name: Some("language".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Required,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] pub language : String ,".to_string()
    );
  }

  #[test]
  fn optional_string_attribute() {
    let attribute = Attribute {
      name: Some("language".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] pub language : Option < String > ,".to_string()
    );
  }

  #[test]
  fn type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] pub kind : Option < String > ,".to_string()
    );
  }
}
