
use yaserde::YaDeserialize;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Qualification {
  #[yaserde(rename="qualified")]
  Qualidified,
  #[yaserde(rename="unqualified")]
  Unqualified,
}

impl Default for Qualification {
  fn default() -> Self {
    Qualification::Unqualified
  }
}

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
  pub element_form_default: Qualification,
  #[yaserde(rename="attributeFormDefault", attribute)]
  pub attribute_form_default: Qualification,
  #[yaserde(rename="import")]
  pub imports: Vec<Import>,
  #[yaserde(rename="element")]
  pub elements: Vec<Element>,
  #[yaserde(rename="complexType")]
  pub complex_type: Vec<ComplexType>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Import {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(attribute)]
  pub namespace: Option<String>,
  #[yaserde(rename="schemaLocation", attribute)]
  pub schema_location: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct ComplexType {
  #[yaserde(attribute)]
  pub name: String,
  pub sequence: Sequence,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Sequence {
  #[yaserde(rename="element")]
  pub elements: Vec<Element>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Element {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename="type", attribute)]
  pub kind: String,
  #[yaserde(rename="minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename="maxOccurs", attribute)]
  pub max_occurences: Option<u64>,
  #[yaserde(rename="complexType")]
  pub complex_type: Vec<Sequence>,
}
