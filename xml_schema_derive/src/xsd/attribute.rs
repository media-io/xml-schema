use crate::xsd::{
  rust_types_mapping::RustTypesMapping, simple_type::SimpleType, Implementation, XsdContext,
};
use heck::SnakeCase;
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

impl Implementation for Attribute {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.name.is_none() {
      return quote!();
    }
    let raw_name = self.name.clone().unwrap();
    let name = raw_name.to_snake_case();

    let name = if name == "type" {
      "kind".to_string()
    } else {
      name
    };

    let field_name = Ident::new(&name, Span::call_site());

    let rust_type = match (
      self.reference.as_ref(),
      self.kind.as_ref(),
      self.simple_type.as_ref(),
    ) {
      (None, Some(kind), None) => RustTypesMapping::get(context, &kind),
      (Some(reference), None, None) => RustTypesMapping::get(context, &reference),
      (None, None, Some(simple_type)) => simple_type.get_type_implementation(context, prefix),
      (_, _, _) => panic!("Not implemented Rust type for: {:?}", self),
    };

    let rust_type = if self.required == Required::Optional {
      quote!(Option<#rust_type>)
    } else {
      quote!(#rust_type)
    };

    let attributes = if name == raw_name {
      quote!(attribute)
    } else {
      quote!(attribute, rename=#raw_name)
    };

    quote!(
      #[yaserde(#attributes)]
      pub #field_name: #rust_type,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn default_required() {
    assert_eq!(Required::default(), Required::Optional);
  }

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

    let implementation = format!(
      "{}",
      attribute.implement(&TokenStream::new(), &None, &context)
    );
    assert_eq!(
      implementation,
      TokenStream::from_str(r#"# [ yaserde ( attribute ) ] pub language : String ,"#)
        .unwrap()
        .to_string()
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

    let implementation = format!(
      "{}",
      attribute.implement(&TokenStream::new(), &None, &context)
    );
    assert_eq!(
      implementation,
      TokenStream::from_str(r#"# [ yaserde ( attribute ) ] pub language : Option < String > ,"#)
        .unwrap()
        .to_string()
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

    let implementation = format!(
      "{}",
      attribute.implement(&TokenStream::new(), &None, &context)
    );
    assert_eq!(
      implementation,
      TokenStream::from_str(
        r#"# [ yaserde ( attribute , rename = "type" ) ] pub kind : Option < String > ,"#
      )
      .unwrap()
      .to_string()
    );
  }

  #[test]
  fn reference_type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: None,
      reference: Some("MyType".to_string()),
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!(
      "{}",
      attribute.implement(&TokenStream::new(), &None, &context)
    );
    assert_eq!(
      implementation,
      TokenStream::from_str(
        r#"# [ yaserde ( attribute , rename = "type" ) ] pub kind : Option < MyType > ,"#
      )
      .unwrap()
      .to_string()
    );
  }

  #[test]
  #[should_panic]
  fn bad_type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: None,
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    attribute.implement(&TokenStream::new(), &None, &context);
  }

  #[test]
  fn attribute_without_name() {
    let attribute = Attribute {
      name: None,
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!(
      "{}",
      attribute.implement(&TokenStream::new(), &None, &context)
    );
    assert_eq!(implementation, "".to_string());
  }
}
