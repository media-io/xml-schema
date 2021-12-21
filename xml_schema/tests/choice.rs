#[macro_use]
extern crate yaserde_derive;

use log::debug;
use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

#[test]
fn choice() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice.xsd")]
  struct ChoiceTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <Parent>
    <XFirstname>John</XFirstname>
  </Parent>
  "#;

  let sample_1: Parent = from_str(xml_1).unwrap();

  let model = Parent {
    x_firstname: Some(Firstname {
      content: "John".to_string(),
      scope: None,
    }),
    x_lastname: None,
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Parent><XFirstname>John</XFirstname></Parent>"#
  );
}
