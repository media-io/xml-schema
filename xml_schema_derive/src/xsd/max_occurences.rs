use std::io::prelude::*;
use xml::reader::XmlEvent;
use yaserde::YaDeserialize;

#[derive(Clone, Debug, PartialEq)]
pub enum MaxOccurences {
  Unbounded,
  Number{value: u32},
}

impl Default for MaxOccurences {
  fn default() -> Self {
    MaxOccurences::Unbounded
  }
}

impl YaDeserialize for MaxOccurences {
  fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {

    if let XmlEvent::StartElement{name, ..} = reader.peek().map_err(|e| e.to_string())? {
      if name.local_name != "MaxOccurences".to_string() {
        return Err("Unable to parse Max Occurences field".to_string());
      }
      let _start_event = reader.next_event();

      let content = reader.next_event().map_err(|e| e.to_string())?;
      
      match content {
        XmlEvent::Characters(value) => {
          if value == "unbounded" {
            Ok(MaxOccurences::Unbounded)
          } else {
            let number = value.parse::<u32>().map_err(|e| e.to_string())?;
            Ok(MaxOccurences::Number{value: number})
          }
        },
        _ => {
          Err("bad content for Max Occurences field".to_string())
        },
      }
    } else {
      Err("Missing start event for Max Occurences field".to_string())
    }
  }
}
