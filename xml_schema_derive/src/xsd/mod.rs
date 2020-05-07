mod attribute;
mod complex_content;
mod complex_type;
mod element;
mod extension;
mod import;
mod list;
mod max_occurences;
mod qualification;
mod restriction;
mod rust_types_mapping;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod union;

use log::info;
use proc_macro2::TokenStream;
use std::fs;
use std::io::Cursor;
use xml::reader::{EventReader, XmlEvent};
use yaserde::de::from_str;

#[derive(Clone, Debug)]
pub struct XsdContext {
  pub xml_schema_prefix: Option<String>,
}

impl XsdContext {
  pub fn new(content: &str) -> Result<Self, String> {
    let cursor = Cursor::new(content);
    let parser = EventReader::new(cursor);

    for xml_element in parser {
      match xml_element {
        Ok(XmlEvent::StartElement { name, .. }) => {
          if name.namespace == Some("http://www.w3.org/2001/XMLSchema".to_string())
            && name.local_name == "schema"
          {
            return Ok(XsdContext {
              xml_schema_prefix: name.prefix,
            });
          }
        }
        Err(_) => {
          break;
        }
        _ => {}
      }
    }

    Err("Bad XML Schema, unable to found schema element.".to_string())
  }
}

#[derive(Clone, Debug)]
pub struct Xsd {
  context: XsdContext,
  schema: schema::Schema,
}

impl Xsd {
  pub fn new(content: &str) -> Result<Self, String> {
    let context = XsdContext::new(content)?;
    let schema: schema::Schema = from_str(content)?;

    Ok(Xsd { context, schema })
  }

  pub fn new_from_file(source: &str) -> Result<Self, String> {
    let content = if source.starts_with("http://") || source.starts_with("https://") {
      info!("Load HTTP schema {}", source);
      reqwest::blocking::get(source)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?
    } else {
      let path = std::env::current_dir().unwrap();
      info!("The current directory is {}", path.display());

      fs::read_to_string(source).map_err(|e| e.to_string())?
    };

    Xsd::new(&content)
  }

  pub fn get_implementation(&self, target_prefix: &Option<String>) -> TokenStream {
    self.schema.get_implementation(target_prefix, &self.context)
  }
}
