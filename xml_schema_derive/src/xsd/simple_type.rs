use crate::xsd::{list::List, restriction::Restriction, union::Union, XsdContext};
use heck::CamelCase;
use log::debug;
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
  #[yaserde(attribute)]
  pub name: String,
  pub restriction: Option<Restriction>,
  pub list: Option<List>,
  pub union: Option<Union>,
}

impl SimpleType {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let struct_name = Ident::new(&self.name.to_camel_case(), Span::call_site());

    if let Some(list) = &self.list {
      return list.get_implementation(context, namespace_definition, prefix, &struct_name);
    }

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #[yaserde(text)]
        pub content: String,
      }
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  static DERIVES: &str =
    "# [ derive ( Clone , Debug , Default , PartialEq , YaDeserialize , YaSerialize ) ] ";

  #[test]
  fn simple_type() {
    let st = SimpleType {
      name: "test".to_string(),
      restriction: None,
      list: None,
      union: None,
    };

    let context = XsdContext {
      xml_schema_prefix: None,
    };

    let ts = st
      .get_implementation(&quote!(), &None, &context)
      .to_string();

    assert!(
      ts == format!(
        "{}pub struct Test {{ # [ yaserde ( text ) ] pub content : String , }}",
        DERIVES
      )
    );
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
