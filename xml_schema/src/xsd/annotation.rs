use crate::xsd::attribute::Attribute;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "annotation"
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
  )]
pub struct Annotation {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(
      rename = "documentation"
      prefix = "xs",
      namespace = "xs: http://www.w3.org/2001/XMLSchema"
    )]
  pub documentation: Vec<String>,
}
