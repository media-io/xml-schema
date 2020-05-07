use crate::xsd::XsdContext;
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Debug)]
pub struct RustTypesMapping {}

impl RustTypesMapping {
  pub fn get(context: &XsdContext, kind: &str) -> TokenStream {
    let items: Vec<&str> = kind.split(':').collect();

    if items.len() == 2 {
      if Some((*items.first().unwrap()).to_string()) == context.xml_schema_prefix {
        RustTypesMapping::basic_type(*items.last().unwrap())
      } else {
        RustTypesMapping::extern_type(*items.last().unwrap())
      }
    } else if items.len() == 1 {
      if context.xml_schema_prefix.is_none() {
        RustTypesMapping::basic_type(*items.last().unwrap())
      } else {
        RustTypesMapping::extern_type(*items.last().unwrap())
      }
    } else {
      unimplemented!();
    }
  }

  pub fn is_xs_string(context: &XsdContext, kind: &str) -> bool {
    let items: Vec<&str> = kind.split(':').collect();

    if items.len() == 2 {
      if Some((*items.first().unwrap()).to_string()) == context.xml_schema_prefix {
        return *items.last().unwrap() == "string";
      }
    } else if items.len() == 1 && context.xml_schema_prefix.is_none() {
      return *items.last().unwrap() == "string";
    }

    false
  }

  fn basic_type(item: &str) -> TokenStream {
    match item {
      "bool" => quote!(bool),
      "boolean" => quote!(bool),
      "positiveInteger" => quote!(u32),
      "byte" => quote!(i8),
      "unsignedByte" => quote!(u8),
      "short" => quote!(i16),
      "unsignedShort" => quote!(u16),
      "int" | "integer" => quote!(i32),
      "unsignedInt" => quote!(u32),
      "long" => quote!(i64),
      "unsignedLong" | "nonNegativeInteger" => quote!(u64),
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
      "anyType" => quote!(String),
      _ => {
        println!("Type {:?} not implemented", item);
        unimplemented!()
      }
    }
  }

  fn extern_type(item: &str) -> TokenStream {
    let struct_name = if item == "" {
      "String".to_string()
    } else {
      item.to_camel_case()
    };

    let struct_name = Ident::new(&struct_name, Span::call_site());

    quote!(#struct_name)
  }
}

#[test]
fn rust_mapping_types() {
  let context = XsdContext {
    xml_schema_prefix: Some("xs".to_string()),
  };

  assert!(RustTypesMapping::get(&context, "xs:boolean").to_string() == "bool");
  assert!(RustTypesMapping::get(&context, "xs:positiveInteger").to_string() == "u32");
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
  assert!(RustTypesMapping::get(&context, "xs:decimal").to_string() == "String");
  assert!(RustTypesMapping::get(&context, "xs:string").to_string() == "String");
  assert!(RustTypesMapping::get(&context, "xs:string").to_string() == "String");

  assert!(RustTypesMapping::get(&context, "other:type").to_string() == "Type");
}
