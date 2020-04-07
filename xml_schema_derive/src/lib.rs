// #[macro_use]
// extern crate log;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate yaserde_derive;

mod attribute;
mod expander;
mod xsd;

#[proc_macro_derive(XmlSchema, attributes(xml_schema))]
pub fn xml_schema_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = match syn::parse(input) {
    Ok(ast) => ast,
    Err(msg) => panic!(msg),
  };

  match expander::expand_derive(&ast) {
    Ok(expanded) => expanded.into(),
    Err(msg) => panic!(msg),
  }
}
