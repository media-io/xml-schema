use crate::{attribute::XmlSchemaAttributes, xsd::Xsd};
use proc_macro2::TokenStream;
use syn::{token::Pub, Visibility};

pub fn expand_derive(attributes: &XmlSchemaAttributes) -> Result<TokenStream, String> {
  let _ = simple_logger::init_with_level(attributes.log_level());
  log::info!("{:?}", attributes);

  let name = "Alksjdfjlksdf".to_string();
  let vis = Visibility::Public(Pub::default());

  let xsd = Xsd::new_from_file(
    name,
    vis,
    &attributes.source,
    &attributes.module_namespace_mappings(),
  )?;
  let generated = xsd.implement(&attributes.target_prefix);

  if let Some(store_generated_code) = &attributes.store_generated_code {
    std::fs::write(store_generated_code, generated.to_string()).map_err(|e| e.to_string())?;
  }

  Ok(generated)
}
