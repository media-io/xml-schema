use crate::xsd::{rust_types_mapping::RustTypesMapping, Implementation, XsdContext};
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

impl Implementation for List {
  fn implement_childs(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    context: &XsdContext,
    struct_name: &Ident,
  ) -> TokenStream {
    let list_type = RustTypesMapping::get(context, &self.item_type);

    quote!(
      #[derive(Clone, Debug, Default, PartialEq)]
      pub struct #struct_name {
        items: Vec<#list_type>
      }

      impl YaDeserialize for #struct_name {
        fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
          loop {
            match reader.next_event()? {
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

      impl YaSerialize for #struct_name {
        fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
          let content =
            self.items.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(" ");

          let data_event = xml::writer::XmlEvent::characters(&content);
          writer.write(data_event).map_err(|e| e.to_string())?;

          Ok(())
        }

        fn serialize_attributes(&self, mut source_attributes: Vec<xml::attribute::OwnedAttribute>, mut source_namespace: xml::namespace::Namespace) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
          Ok((source_attributes, source_namespace))
        }
      }
    )
  }
}
