use crate::xsd::attribute::Attribute;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attributeGroup",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AttributeGroup {
  #[yaserde(prefix = "xs", attribute)]
  pub name: Option<String>,
  #[yaserde(rename = "ref", attribute)]
  pub reference: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  // #[yaserde(rename = "attributeGroup")]
  // pub attribute_group: Vec<AttributeGroup>,
}
