use crate::xsd::extension::Extension;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleContent {
  #[yaserde(prefix = "xs", rename = "extension")]
  pub extension: Extension,
}
