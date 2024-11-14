#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Union {
  #[yaserde(rename = "memberTypes", attribute)]
  pub member_types: String,
}
