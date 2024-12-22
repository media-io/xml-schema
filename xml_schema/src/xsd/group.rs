use crate::xsd::sequence::Sequence;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Group {
  #[yaserde(attribute)]
  pub name: Option<String>,
  #[yaserde(attribute, rename = "ref")]
  pub reference: Option<String>,
  #[yaserde()]
  pub sequence: Option<Sequence>,
}
