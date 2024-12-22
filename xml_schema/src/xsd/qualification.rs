#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Qualification {
  #[yaserde(rename = "qualified")]
  Qualidified,
  #[default]
  #[yaserde(rename = "unqualified")]
  Unqualified,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn default_qualification() {
    assert_eq!(Qualification::default(), Qualification::Unqualified);
  }
}
