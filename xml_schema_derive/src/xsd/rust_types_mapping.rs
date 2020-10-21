use crate::xsd::XsdContext;
use heck::CamelCase;
use proc_macro2::TokenStream;
use syn::{parse_str, TypePath};

#[derive(Debug)]
pub struct RustTypesMapping {}

impl RustTypesMapping {
  pub fn get(context: &XsdContext, kind: &str) -> TokenStream {
    let items: Vec<&str> = kind.split(':').collect();

    if items.len() == 2 {
      if context.match_xml_schema_prefix(*items.first().unwrap()) {
        RustTypesMapping::basic_type(*items.last().unwrap())
      } else {
        RustTypesMapping::extern_type(context, items)
      }
    } else if items.len() == 1 {
      if context.has_xml_schema_prefix() {
        RustTypesMapping::extern_type(context, items)
      } else {
        RustTypesMapping::basic_type(*items.last().unwrap())
      }
    } else {
      panic!("Unknown type {}", kind)
    }
  }

  pub fn is_xs_string(context: &XsdContext, kind: &str) -> bool {
    let items: Vec<&str> = kind.split(':').collect();

    if items.len() == 2 {
      if context.match_xml_schema_prefix(*items.first().unwrap()) {
        return *items.last().unwrap() == "string";
      }
    } else if items.len() == 1 && !context.has_xml_schema_prefix() {
      return *items.last().unwrap() == "string";
    }

    false
  }

  fn basic_type(item: &str) -> TokenStream {
    match item {
      "bool" => quote!(bool),
      "boolean" => quote!(bool),
      "positiveInteger" => quote!(u64),
      "byte" => quote!(i8),
      "unsignedByte" => quote!(u8),
      "short" => quote!(i16),
      "unsignedShort" => quote!(u16),
      "int" | "integer" => quote!(i32),
      "unsignedInt" => quote!(u32),
      "long" => quote!(i64),
      "unsignedLong" | "nonNegativeInteger" => quote!(u64),
      "float" => quote!(f32),
      "double" => quote!(f64),
      "decimal" => quote!(String), // TODO replace with f64
      "string" => quote!(String),
      "normalizedString" => quote!(String),
      "anyURI" => quote!(String),
      "token" => quote!(String),
      "language" => quote!(String),
      "hexBinary" => quote!(String),
      "dateTime" => quote!(String),
      "base64Binary" => quote!(String),
      "duration" => quote!(String),
      "gYear" => quote!(u16),
      "ID" => quote!(String),
      "IDREF" => quote!(String),
      "IDREFS" => quote!(String),
      "anyType" => quote!(String),
      _ => panic!("Type {:?} not implemented", item),
    }
  }

  fn extern_type(context: &XsdContext, items: Vec<&str>) -> TokenStream {
    let struct_name = if *items.last().unwrap() == "" {
      "String".to_string()
    } else {
      (*items.last().unwrap().replace(".", "_").to_camel_case()).to_string()
    };

    let default_module = context
      .get_module("")
      .map(|module| format!("{}::", module))
      .unwrap_or_else(|| "".to_string());

    let module = if items.len() == 2 {
      let prefix = items.first().unwrap();
      if let Some(module) = context.get_module(&prefix) {
        module + "::"
      } else {
        default_module
      }
    } else {
      default_module
    };

    let struct_name = format!("{}{}", module, struct_name);
    let struct_name = parse_str::<TypePath>(&struct_name).unwrap();
    quote!(#struct_name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeMap;

  #[test]
  fn rust_mapping_types() {
    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    assert!(RustTypesMapping::get(&context, "xs:boolean").to_string() == "bool");
    assert!(RustTypesMapping::get(&context, "xs:positiveInteger").to_string() == "u64");
    assert!(RustTypesMapping::get(&context, "xs:byte").to_string() == "i8");
    assert!(RustTypesMapping::get(&context, "xs:unsignedByte").to_string() == "u8");
    assert!(RustTypesMapping::get(&context, "xs:short").to_string() == "i16");
    assert!(RustTypesMapping::get(&context, "xs:unsignedShort").to_string() == "u16");
    assert!(RustTypesMapping::get(&context, "xs:int").to_string() == "i32");
    assert!(RustTypesMapping::get(&context, "xs:integer").to_string() == "i32");
    assert!(RustTypesMapping::get(&context, "xs:unsignedInt").to_string() == "u32");
    assert!(RustTypesMapping::get(&context, "xs:long").to_string() == "i64");
    assert!(RustTypesMapping::get(&context, "xs:unsignedLong").to_string() == "u64");
    assert!(RustTypesMapping::get(&context, "xs:nonNegativeInteger").to_string() == "u64");
    assert!(RustTypesMapping::get(&context, "xs:float").to_string() == "f32");
    assert!(RustTypesMapping::get(&context, "xs:double").to_string() == "f64");
    assert!(RustTypesMapping::get(&context, "xs:decimal").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:string").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:string").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:ID").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:IDREF").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:IDREFS").to_string() == "String");
    assert!(RustTypesMapping::get(&context, "xs:anyType").to_string() == "String");

    assert!(RustTypesMapping::get(&context, "other:type").to_string() == "Type");

    let context =
      XsdContext::new(r#"<schema xmlns="http://www.w3.org/2001/XMLSchema"></schema>"#).unwrap();

    assert!(RustTypesMapping::get(&context, "boolean").to_string() == "bool");
  }

  #[test]
  #[should_panic]
  fn rust_bad_mapping_type() {
    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    RustTypesMapping::get(&context, "xs:unknown");
  }

  #[test]
  fn extern_types() {
    let context = XsdContext::new(
      r#"
      <xs:schema
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        xmlns:example="http://example.com"
        >
      </xs:schema>
    "#,
    )
    .unwrap();

    let mut mapping = BTreeMap::new();
    mapping.insert(
      "http://example.com".to_string(),
      "rust_example_module".to_string(),
    );

    let context = context.with_module_namespace_mappings(&mapping);
    assert_eq!(
      RustTypesMapping::get(&context, "example:MyType").to_string(),
      "rust_example_module :: MyType"
    );

    assert_eq!(
      RustTypesMapping::get(&context, "example:").to_string(),
      "rust_example_module :: String"
    );
  }

  #[test]
  fn extern_types_in_default_namespace() {
    let context = XsdContext::new(
      r#"
      <xs:schema
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        xmlns="http://example.com"
        >
      </xs:schema>
    "#,
    )
    .unwrap();

    let mut mapping = BTreeMap::new();
    mapping.insert(
      "http://example.com".to_string(),
      "rust_example_module".to_string(),
    );

    let context = context.with_module_namespace_mappings(&mapping);
    assert_eq!(
      RustTypesMapping::get(&context, "MyType").to_string(),
      "rust_example_module :: MyType"
    );

    assert_eq!(
      RustTypesMapping::get(&context, "").to_string(),
      "rust_example_module :: String"
    );
  }

  #[test]
  fn is_xs_string() {
    let context = XsdContext::new(
      r#"
      <xs:schema
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        xmlns="http://example.com"
        >
      </xs:schema>
    "#,
    )
    .unwrap();

    assert_eq!(RustTypesMapping::is_xs_string(&context, "xs:string"), true);
    assert_eq!(RustTypesMapping::is_xs_string(&context, "MyType"), false);

    let context = XsdContext::new(
      r#"
      <schema
        xmlns="http://www.w3.org/2001/XMLSchema"
        xmlns:example="http://example.com"
        >
      </schema>
    "#,
    )
    .unwrap();

    assert_eq!(RustTypesMapping::is_xs_string(&context, "string"), true);
    assert_eq!(
      RustTypesMapping::is_xs_string(&context, "example:MyType"),
      false
    );
  }
}
