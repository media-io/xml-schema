use std::collections::BTreeMap;
use std::io::Cursor;
use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Debug)]
pub struct XsdContext {
  module_namespace_mappings: BTreeMap<String, String>,
  pub namespace: Namespace,
  xml_schema_prefix: Option<String>,
}

impl XsdContext {
  pub fn new(content: &str) -> Result<Self, String> {
    let cursor = Cursor::new(content);
    let parser = EventReader::new(cursor);

    for xml_element in parser {
      match xml_element {
        Ok(XmlEvent::StartElement {
          name, namespace, ..
        }) => {
          if name.namespace == Some("http://www.w3.org/2001/XMLSchema".to_string())
            && name.local_name == "schema"
          {
            return Ok(XsdContext {
              module_namespace_mappings: BTreeMap::new(),
              namespace,
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

  pub fn with_module_namespace_mappings(
    mut self,
    module_namespace_mappings: &BTreeMap<String, String>,
  ) -> Self {
    self.module_namespace_mappings = module_namespace_mappings.clone();
    self
  }

  pub fn has_xml_schema_prefix(&self) -> bool {
    self.xml_schema_prefix.is_some()
  }

  pub fn match_xml_schema_prefix(&self, value: &str) -> bool {
    self.xml_schema_prefix == Some(value.to_string())
  }

  pub fn get_module(&self, prefix: &str) -> Option<String> {
    self
      .namespace
      .get(prefix)
      .map(|namespace| {
        self
          .module_namespace_mappings
          .get(namespace)
          .map(|module| module.to_owned())
      })
      .unwrap_or_else(|| None)
  }
}
