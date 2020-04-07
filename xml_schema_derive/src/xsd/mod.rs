mod complex_type;
mod element;
mod import;
mod max_occurences;
mod qualification;
mod sequence;

use proc_macro2::TokenStream;
use yaserde::de::from_str;
use yaserde::YaDeserialize;
use std::fs;
use std::io::prelude::*;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Xsd {
  #[yaserde(rename="targetNamespace", attribute)]
  pub target_namespace: Option<String>,
  #[yaserde(rename="elementFormDefault", attribute)]
  pub element_form_default: qualification::Qualification,
  #[yaserde(rename="attributeFormDefault", attribute)]
  pub attribute_form_default: qualification::Qualification,
  #[yaserde(rename="import")]
  pub imports: Vec<import::Import>,
  #[yaserde(rename="element")]
  pub elements: Vec<element::Element>,
  #[yaserde(rename="complexType")]
  pub complex_type: Vec<complex_type::ComplexType>,
}

impl Xsd {
  pub fn new(content: &str) -> Result<Self, String> {
    from_str(&content)
  }

  pub fn new_from_file(source: &str) -> Result<Self, String> {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let content = fs::read_to_string(source).map_err(|e| e.to_string())?;
    Xsd::new(&content)
  }

  pub fn get_implementation(&self) -> TokenStream {
    let elements: TokenStream = self.elements
      .iter()
      .map(|element| {
        element.get_implementation()
      })
      .collect();


    let complex_types: TokenStream = self.complex_type
      .iter()
      .map(|complex_type| {
        complex_type.get_implementation()
      })
      .collect();

    quote!(
      #complex_types
      #elements
    )
  }
}
