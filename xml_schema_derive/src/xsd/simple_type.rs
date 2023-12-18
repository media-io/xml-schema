use crate::xsd::{list::List, restriction::Restriction, union::Union, Implementation, XsdContext};
use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
  #[yaserde(attribute)]
  pub name: String,
  pub restriction: Option<Restriction>,
  pub list: Option<List>,
  pub union: Option<Union>,
}

impl Implementation for SimpleType {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name.to_upper_camel_case(), Span::call_site());

    if let Some(list) = &self.list {
      return list.implement_childs(namespace_definition, prefix, context, &struct_name);
    }

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #[yaserde(text)]
        pub content: std::string::String,
      }
    )
  }
}

impl SimpleType {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    if let Some(restriction) = &self.restriction {
      restriction.get_type_implementation(context, prefix)
    } else {
      panic!("No restriction for this simple type {:?}", self);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  static DERIVES: &str =
    "# [derive (Clone , Debug , Default , PartialEq , yaserde_derive :: YaDeserialize , yaserde_derive :: YaSerialize)] ";

  #[test]
  fn simple_type() {
    let st = SimpleType {
      name: "test".to_string(),
      restriction: None,
      list: None,
      union: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = st.implement(&quote!(), &None, &context);

    let expected = TokenStream::from_str(&format!(
      r#"{DERIVES}
        pub struct Test {{
          #[yaserde(text)]
          pub content: std::string::String,
        }}"#,
    ))
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  // <!-- Whitespace-separated list of strings -->
  // <xs:simpleType name="StringVectorType">
  //   <xs:list itemType="xs:string"/>
  // </xs:simpleType>

  // <!-- Whitespace-separated list of unsigned integers -->
  // <xs:simpleType name="UIntVectorType">
  //   <xs:list itemType="xs:unsignedInt"/>
  // </xs:simpleType>

  // #[test]
  // fn list_type() {
  //   let st = SimpleType {
  //     name: "string-list".to_string(),
  //     restriction: None,
  //     list: Some(List{
  //       item_type: "xs:string".to_string()
  //     }),
  //     union: None,
  //   };

  //   let context = XsdContext {
  //     xml_schema_prefix: Some("xs".to_string()),
  //   };

  //   let ts = st
  //     .get_implementation(&quote!(), &None, &context)
  //     .to_string();
  //   println!("{}", ts);
  //   assert!(ts == format!("{}pub struct StringList {{ # [ yaserde ( text ) ] pub content : String , }}", DERIVES));
  // }
}
