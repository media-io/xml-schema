mod annotation;
mod attribute;
mod attribute_group;
mod choice;
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
mod rust_types_mapping;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod union;
mod xsd_context;

use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use std::collections::BTreeMap;
use std::fs;
use syn::Visibility;
use xsd_context::XsdContext;
use yaserde::de::from_str;

trait Implementation {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
  ) -> TokenStream {
    unimplemented!()
  }

  fn implement_childs(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
    _struct_name: &Ident,
  ) -> TokenStream {
    unimplemented!()
  }
}

#[derive(Clone, Debug)]
pub struct Xsd {
  name: String,
  vis: Visibility,
  context: XsdContext,
  schema: schema::Schema,
}

impl Xsd {
  pub fn new(
    name: String,
    vis: Visibility,
    content: &str,
    module_namespace_mappings: &BTreeMap<String, String>,
  ) -> Result<Self, String> {
    let context = XsdContext::new(content)?;
    let context = context.with_module_namespace_mappings(module_namespace_mappings);
    let schema: schema::Schema = from_str(content)?;

    Ok(Xsd {
      name,
      vis,
      context,
      schema,
    })
  }

  pub fn new_from_file(
    name: String,
    vis: Visibility,
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

    Xsd::new(name, vis, &content, module_namespace_mappings)
  }

  pub fn implement(&self, target_prefix: &Option<String>) -> TokenStream {
    let schema = self
      .schema
      .implement(&TokenStream::new(), target_prefix, &self.context);

    let mod_name = format_ident!("{}", self.name.to_snake_case());
    let vis = &self.vis;

    quote! {
        mod #mod_name {
            #schema
        }

        #vis use #mod_name::*;
    }
  }
}
