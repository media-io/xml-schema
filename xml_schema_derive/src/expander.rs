use crate::attribute::XmlSchemaAttribute;
use crate::xsd::Xsd;
use proc_macro2::TokenStream;

pub fn expand_derive(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let attributes = XmlSchemaAttribute::parse(&ast.attrs);
  let xsd = Xsd::new_from_file(&attributes.source)?;
  let generated = xsd.get_implementation(&attributes.target_prefix);

  if let Some(store_generated_code) = &attributes.store_generated_code {
    std::fs::write(store_generated_code, generated.to_string()).map_err(|e| e.to_string())?;
  }

  Ok(generated)
}
