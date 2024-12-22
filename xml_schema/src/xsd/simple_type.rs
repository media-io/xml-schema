use crate::xsd::{list::List, restriction::Restriction, union::Union};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
  #[yaserde(attribute)]
  pub name: String,
  pub restriction: Option<Restriction>,
  pub list: Option<List>,
  pub union: Option<Union>,
}
