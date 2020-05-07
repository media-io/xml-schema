use crate::xsd::{rust_types_mapping::RustTypesMapping, XsdContext};
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
  pub name: String,
  #[yaserde(rename = "type", attribute)]
  pub kind: String,
  // #[yaserde(attribute)]
  // pub default: Option<String>,
  // #[yaserde(attribute)]
  // pub fixed: Option<String>,
  #[yaserde(rename = "use", attribute)]
  pub required: Required,
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
    let name = if self.name == "type" {
      "kind".to_string()
    } else {
      self.name.clone()
    };

    let field_name = Ident::new(&name, Span::call_site());
    let rust_type = RustTypesMapping::get(context, &self.kind);

    let rust_type = if self.required == Required::Optional {
      quote!(Option<#rust_type>)
    } else {
      quote!(#rust_type)
    };

    quote!(
      #[yaserde(attribute)]
      #field_name: #rust_type,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn string_attribute() {
    let attribute = Attribute {
      name: "language".to_string(),
      kind: "xs:string".to_string(),
      required: Required::Required,
    };

    let context = XsdContext {
      xml_schema_prefix: Some("xs".to_string()),
    };

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] language : String ,".to_string()
    );
  }

  #[test]
  fn optional_string_attribute() {
    let attribute = Attribute {
      name: "language".to_string(),
      kind: "xs:string".to_string(),
      required: Required::Optional,
    };

    let context = XsdContext {
      xml_schema_prefix: Some("xs".to_string()),
    };

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] language : Option < String > ,".to_string()
    );
  }

  #[test]
  fn type_attribute() {
    let attribute = Attribute {
      name: "type".to_string(),
      kind: "xs:string".to_string(),
      required: Required::Optional,
    };

    let context = XsdContext {
      xml_schema_prefix: Some("xs".to_string()),
    };

    let implementation = format!("{}", attribute.get_implementation(&context));
    assert_eq!(
      implementation,
      "# [ yaserde ( attribute ) ] kind : Option < String > ,".to_string()
    );
  }
}
