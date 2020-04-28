use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Debug)]
pub struct RustTypesMapping {}

impl RustTypesMapping {
  pub fn get(kind: &str) -> TokenStream {
    match kind {
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
      "xs:nonNegativeInteger" => quote!(u64),
      // "xs:decimal" => quote!(f64),
      "xs:decimal" => quote!(String),
      "xs:string" => quote!(String),
      "xs:normalizedString" => quote!(String),
      "xs:anyURI" => quote!(String),
      "xs:token" => quote!(String),
      "xs:language" => quote!(String),
      "xs:hexBinary" => quote!(String),
      "xs:dateTime" => quote!(String),
      "xs:base64Binary" => quote!(String),
      "base64Binary" => quote!(String),
      "xs:duration" => quote!(String),
      "xs:gYear" => quote!(u16),
      "string" => quote!(String),
      "integer" => quote!(i32),
      "ID" => quote!(String),
      "anyURI" => quote!(String),
      _ => {
        let v: Vec<&str> = kind.split(':').collect();
        let struct_name = (*v.last().unwrap()).to_string();

        let struct_name = if struct_name == "" {
          "String".to_string()
        } else {
          struct_name.to_camel_case()
        };

        let struct_name = Ident::new(&struct_name, Span::call_site());

        quote!(#struct_name)
      }
    }
  }
}

#[test]
fn rust_mapping_types() {
  assert!(RustTypesMapping::get("xs:boolean").to_string() == "bool");
  assert!(RustTypesMapping::get("xs:positiveInteger").to_string() == "u32");
  assert!(RustTypesMapping::get("xs:byte").to_string() == "i8");
  assert!(RustTypesMapping::get("xs:unsignedByte").to_string() == "u8");
  assert!(RustTypesMapping::get("xs:short").to_string() == "i16");
  assert!(RustTypesMapping::get("xs:unsignedShort").to_string() == "u16");
  assert!(RustTypesMapping::get("xs:int").to_string() == "i32");
  assert!(RustTypesMapping::get("xs:integer").to_string() == "i32");
  assert!(RustTypesMapping::get("xs:unsignedInt").to_string() == "u32");
  assert!(RustTypesMapping::get("xs:long").to_string() == "i64");
  assert!(RustTypesMapping::get("xs:unsignedLong").to_string() == "u64");
  assert!(RustTypesMapping::get("xs:nonNegativeInteger").to_string() == "u64");
  assert!(RustTypesMapping::get("xs:decimal").to_string() == "String");
  assert!(RustTypesMapping::get("xs:string").to_string() == "String");
}
