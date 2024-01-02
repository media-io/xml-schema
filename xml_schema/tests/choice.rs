#[macro_use]
extern crate yaserde_derive;

use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[test]
fn choice() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice.xsd")]
  struct ChoiceTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <person>
    <firstname>John</firstname>
  </person>
  "#;

  let sample_1: Person = from_str(xml_1).unwrap();

  let model = Person {
    firstname: Some(Firstname {
      content: "John".to_string(),
      scope: None,
    }),
    lastname: None,
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Person><firstname>John</firstname></Person>"#
  );
}

#[test]
fn choice_sequence() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice_sequence.xsd")]
  struct ChoiceTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <person>
    <name>Doe</name>
    <firstname>John</firstname>
  </person>
  "#;

  let sample_1: Person = from_str(xml_1).unwrap();

  let model = Person {
    name: "Doe".to_string(),
    firstname: Some(Firstname {
      content: "John".to_string(),
      scope: None,
    }),
    lastname: None,
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Person><name>Doe</name><firstname>John</firstname></Person>"#
  );
}

#[test]
fn choice_multiple() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice_multiple.xsd")]
  struct ChoiceTypeSchema;

  let xml_1 = r#"
  <?xml version="1.0" encoding="UTF-8"?>
  <person>
    <firstname>John</firstname>
  </person>
  "#;

  let sample_1: Person = from_str(xml_1).unwrap();

  let model = Person {
    firstnames: vec!["John".to_string()],
    lastnames: vec![],
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Person><firstname>John</firstname></Person>"#
  );
}
