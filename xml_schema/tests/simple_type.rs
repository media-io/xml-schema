#[macro_use]
extern crate yaserde_derive;

use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

#[test]
fn simple_type_string() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(
    source = "xml_schema/tests/simple_type_string.xsd",
    target_prefix = "st"
  )]
  struct SimpleTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <Sample-type>
    Test content
  </Sample-type>
  "#;

  let sample_1: SampleType = from_str(xml_1).unwrap();

  let model = SampleType {
    content: "Test content".to_string(),
  };

  assert_eq!(sample_1, model);
}

#[test]
fn simple_type_list() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/simple_type_list.xsd")]
  struct SimpleTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <BaseType strings="value1 value2" integers="3 6" booleans="true false" />
  "#;

  let sample_1: BaseType = from_str(xml_1).unwrap();

  let model = BaseType {
    strings: Some(StringList {
      items: vec!["value1".to_string(), "value2".to_string()],
    }),
    integers: Some(IntegerList { items: vec![3, 6] }),
    booleans: Some(BooleanList {
      items: vec![true, false],
    }),
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><BaseType strings="value1 value2" integers="3 6" booleans="true false" />"#
  );
}
