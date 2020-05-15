use crate::attribute::XmlSchemaAttribute;
use crate::xsd::Xsd;
use log::info;
use proc_macro2::TokenStream;

pub fn expand_derive(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let attributes = XmlSchemaAttribute::parse(&ast.attrs);
  let _ = simple_logger::init_with_level(attributes.log_level);

  info!("{:?}", attributes);

  let xsd = Xsd::new_from_file(&attributes.source, &attributes.module_namespace_mappings)?;
  let generated = xsd.get_implementation(&attributes.target_prefix);

  if let Some(store_generated_code) = &attributes.store_generated_code {
    std::fs::write(store_generated_code, generated.to_string()).map_err(|e| e.to_string())?;
  }

  Ok(generated)
}
