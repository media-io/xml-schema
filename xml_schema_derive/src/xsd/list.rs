use crate::xsd::{rust_types_mapping::RustTypesMapping, XsdContext};
use log::debug;
use proc_macro2::{Ident, TokenStream};
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct List {
  #[yaserde(rename = "itemType", attribute)]
  pub item_type: String,
}

impl List {
  pub fn get_implementation(
    &self,
    context: &XsdContext,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    struct_name: &Ident,
  ) -> TokenStream {
    let list_type = RustTypesMapping::get(context, &self.item_type);

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, YaSerialize)]
      pub struct #struct_name {
        items: Vec<#list_type>
      }

      impl YaDeserialize for #struct_name {
        fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
          loop {
            let event = reader.next_event()?;
            println!("{:?}", event);
            match event {
              xml::reader::XmlEvent::StartElement{..} => {}
              xml::reader::XmlEvent::Characters(ref text_content) => {
                let items: Vec<#list_type> =
                  text_content
                    .split(' ')
                    .map(|item| item.to_owned())
                    .map(|item| item.parse().unwrap())
                    .collect();

                return Ok(#struct_name {items});
              }
              _ => {break;}
            }
          }

          Err("Unable to parse attribute".to_string())
        }
      }
    )
  }
}
