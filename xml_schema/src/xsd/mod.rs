mod annotation;
mod attribute;
mod attribute_group;
mod complex_content;
mod complex_type;
mod element;
mod extension;
mod group;
mod import;
mod list;
mod max_occurences;
mod qualification;
mod restriction;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod union;
mod xsd_context;

pub use annotation::*;
pub use attribute::*;
pub use attribute_group::*;
pub use complex_content::*;
pub use complex_type::*;
pub use element::*;
pub use extension::*;
pub use group::*;
pub use import::*;
pub use list::*;
pub use max_occurences::*;
pub use qualification::*;
pub use restriction::*;
pub use schema::*;
pub use sequence::*;
pub use simple_content::*;
pub use simple_type::*;
pub use union::*;
pub use xsd_context::*;

use std::collections::BTreeMap;
use std::fs;
use yaserde::de::from_str;

#[derive(Clone, Debug)]
pub struct Xsd {
  pub name: String,
  pub context: XsdContext,
  pub schema: schema::Schema,
}

impl Xsd {
  pub fn new(
    name: String,
    content: &str,
    module_namespace_mappings: &BTreeMap<String, String>,
  ) -> Result<Self, String> {
    let context = XsdContext::new(content)?;
    let context = context.with_module_namespace_mappings(module_namespace_mappings);
    let schema: schema::Schema = from_str(content)?;

    Ok(Xsd {
      name,
      context,
      schema,
    })
  }

  pub fn new_from_file(
    name: String,
    source: &str,
    module_namespace_mappings: &BTreeMap<String, String>,
  ) -> Result<Self, String> {
    let content = if source.starts_with("http://") || source.starts_with("https://") {
      log::info!("Load HTTP schema {}", source);
      reqwest::blocking::get(source)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?
    } else {
      let path = std::env::current_dir().unwrap();
      log::info!("The current directory is {}", path.display());

      fs::read_to_string(source).map_err(|e| e.to_string())?
    };

    // skip BOM header, can be present on some files
    let content = if content.as_bytes()[0..3] == [0xef, 0xbb, 0xbf] {
      content[3..].to_owned()
    } else {
      content
    };

    Xsd::new(name, &content, module_namespace_mappings)
  }
}
