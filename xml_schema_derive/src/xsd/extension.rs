use crate::xsd::{
  attribute::Attribute, rust_types_mapping::RustTypesMapping, sequence::Sequence, XsdContext,
};
use log::debug;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root = "extension",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
  #[yaserde(attribute)]
  pub base: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "sequence")]
  pub sequences: Vec<Sequence>,
}

impl Extension {
  pub fn get_implementation(&self, context: &XsdContext) -> TokenStream {
    let rust_type = RustTypesMapping::get(context, &self.base);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.get_implementation(context))
      .collect();

    let inner_attribute = if format!("{}", rust_type) == "String" {
      quote!(#[yaserde(text)])
    } else {
      quote!()
    };

    quote!(
      #inner_attribute
      pub content: #rust_type,
      #attributes
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn extension() {
    let st = Extension {
      base: "xs:string".to_string(),
      attributes: vec![],
      sequences: vec![],
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let ts = st.get_implementation(&context).to_string();
    assert!(ts == "# [ yaserde ( text ) ] pub content : String ,");
  }

  #[test]
  fn extension_with_attributes() {
    use crate::xsd::attribute::Required;

    let st = Extension {
      base: "xs:string".to_string(),
      attributes: vec![
        Attribute {
          name: Some("attribute_1".to_string()),
          kind: Some("xs:string".to_string()),
          reference: None,
          required: Required::Required,
        },
        Attribute {
          name: Some("attribute_2".to_string()),
          kind: Some("xs:boolean".to_string()),
          reference: None,
          required: Required::Optional,
        },
      ],
      sequences: vec![],
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let ts = st.get_implementation(&context).to_string();
    assert!(ts == "# [ yaserde ( text ) ] pub content : String , # [ yaserde ( attribute ) ] pub attribute_1 : String , # [ yaserde ( attribute ) ] pub attribute_2 : Option < bool > ,");
  }
}
