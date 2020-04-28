#[macro_use]
extern crate yaserde_derive;

use log::debug;
use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;
use yaserde::{YaDeserialize, YaSerialize};

#[test]
fn simple_type_string() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(
      source = "xml_schema/tests/simple_type_string.xsd",
      target_prefix = "st"
      store_generated_code = "st.rs"
  )]
  struct SimpleTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <Sample-type>
    Test content
  </Sample-type>
  "#;

  let sample_1: SampleType = from_str(xml_1).unwrap();

  assert!(
    sample_1
      == SampleType {
        content: "Test content".to_string()
      }
  );
}
