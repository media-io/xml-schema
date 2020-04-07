
use crate::attribute::XmlSchemaAttribute;
use crate::xsd::Xsd;
use proc_macro2::TokenStream;

pub fn expand_derive(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let attributes = XmlSchemaAttribute::parse(&ast.attrs);
  let xsd = Xsd::new_from_file(&attributes.source)?;
  let generated = xsd.get_implementation();

  println!("{}", generated.to_string());
  Ok(generated)
}
