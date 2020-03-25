
use crate::attribute::XmlSchemaAttribute;
use crate::xsd::Xsd;

use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use yaserde::de::from_str;
use syn::Ident;

use std::io::prelude::*;
use std::fs::File;

pub fn expand_derive(ast: &syn::DeriveInput) -> Result<TokenStream, String> {

  let attributes = XmlSchemaAttribute::parse(&ast.attrs);

  let mut f = File::open(attributes.source).map_err(|e| e.to_string())?;

  let mut content = String::new();
  f.read_to_string(&mut content).map_err(|e| e.to_string())?;
  let xsd: Xsd = from_str(&content)?;

  println!("{:?}", xsd.imports);

  let generated: TokenStream = xsd.elements.clone()
    .iter()
    .map(|element| {
      let struct_name = Ident::new(&element.name, Span::call_site());

      let i: Vec<&str> = element.kind.split(":").collect();
      let complex_type_name = i[1];

      let struct_attributes: TokenStream =
        xsd.complex_type.iter().map(|complex_type| {
          if complex_type.name == complex_type_name {
            let sub_elements: TokenStream = 
              complex_type.sequence.elements.iter().map(|sub_element| {
                let attribute_name = Ident::new(&sub_element.name.to_snake_case(), Span::call_site());
                let yaserde_rename = &sub_element.name;

                quote!{
                  #[yaserde(rename=#yaserde_rename)]
                  pub #attribute_name: u64,
                }
              })
              .collect();

            Some(sub_elements)
          } else {
            None
          }
        })
        .filter_map(|x| x)
        .collect();

      quote! {
        #[derive(Clone, PartialEq, YaDeserialize, YaSerialize)]
        pub struct #struct_name {
          #struct_attributes
        }
      }
    })
    .collect();

  println!("{}", generated.to_string());

  Ok(generated)
}
