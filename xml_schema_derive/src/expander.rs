use crate::{attribute::XmlSchemaAttributes, xsd::implement_xsd};
use proc_macro2::TokenStream;
use syn::{token::Pub, Visibility};

use xml_schema::Xsd;

pub fn expand_derive(attributes: &XmlSchemaAttributes) -> Result<TokenStream, String> {
  let _ = simple_logger::init_with_level(attributes.log_level());
  log::info!("{:?}", attributes);

  let vis = Visibility::Public(Pub::default());

  let xsd = Xsd::new_from_file(
    attributes.module_name(),
    &attributes.source,
    &attributes.module_namespace_mappings(),
  )?;
  let generated = implement_xsd(&xsd, vis, &attributes.target_prefix);

  if let Some(store_generated_code) = &attributes.store_generated_code {
    std::fs::write(store_generated_code, generated.to_string()).map_err(|e| e.to_string())?;
  }

  Ok(generated)
}
