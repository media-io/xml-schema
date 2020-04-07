// use heck::CamelCase;
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
      _ => {
        let v: Vec<&str> = kind.split(':').collect();
        let mut struct_name = (*v.last().unwrap()).to_string(); //.to_camel_case();

        if struct_name == "" {
          struct_name = "String".to_string();
        }
        let struct_name = Ident::new(&struct_name, Span::call_site());

        quote!(#struct_name)
      }
    }
  }
}
