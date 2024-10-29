use crate::xsd::{
  attribute::Attribute, group::Group, rust_types_mapping::RustTypesMapping, sequence::Sequence,
  Implementation, XsdContext,
};
use proc_macro2::TokenStream;

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
  #[yaserde(rename = "group")]
  pub group: Option<Group>,
}

impl Implementation for Extension {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let rust_type = RustTypesMapping::get(context, &self.base);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.implement(namespace_definition, prefix, context))
      .collect();

    let inner_attribute = if format!("{rust_type}") == "String" {
      quote!(#[yaserde(text)])
    } else {
      TokenStream::new()
    };

    let sequences: TokenStream = self
      .sequences
      .iter()
      .map(|sequence| sequence.implement(namespace_definition, prefix, context))
      .collect();

    quote!(
      #inner_attribute
      pub base: #rust_type,
      #attributes
      #sequences
    )
  }
}

impl Extension {
  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let rust_type = RustTypesMapping::get(context, &self.base);

    let group_content = self
      .group
      .as_ref()
      .map(|group| {
        let group_type = group.get_type_implementation(context, prefix);

        quote!(
          #[serde(flatten)]
          pub extension : #group_type,
        )
      })
      .unwrap_or_default();

    let sequences: TokenStream = self
      .sequences
      .iter()
      .map(|sequence| sequence.get_field_implementation(context, prefix))
      .collect();

    quote!(
      pub base : #rust_type,
      #group_content
      #sequences
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn extension() {
    let st = Extension {
      base: "xs:string".to_string(),
      attributes: vec![],
      sequences: vec![],
      group: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = st.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(text)]
        pub base: String,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
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
          simple_type: None,
        },
        Attribute {
          name: Some("attribute_2".to_string()),
          kind: Some("xs:boolean".to_string()),
          reference: None,
          required: Required::Optional,
          simple_type: None,
        },
      ],
      sequences: vec![],
      group: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = st.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(text)]
        pub base: String,
        #[yaserde(attribute)]
        pub attribute_1: String,
        #[yaserde(attribute)]
        pub attribute_2: Option<bool> ,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn extension_with_sequences() {
    use crate::xsd::complex_content::ComplexContent;
    use crate::xsd::complex_type::ComplexType;
    use crate::xsd::element::Element;

    /*
    <xs:complexType name="fullpersoninfo">
      <xs:complexContent>
        <xs:extension base="personinfo">
          <xs:sequence>
            <xs:element name="address" type="xs:string"/>
            <xs:element name="city" type="xs:string"/>
            <xs:element name="country" type="xs:string"/>
          </xs:sequence>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
    */

    let extension = Extension {
      base: "personinfo".to_string(),
      attributes: vec![],
      sequences: vec![Sequence {
        elements: vec![
          Element {
            name: "address".to_string(),
            kind: Some("xs:string".to_string()),
            refers: None,
            min_occurences: None,
            max_occurences: None,
            complex_type: None,
            simple_type: None,
            annotation: None,
          },
          Element {
            name: "city".to_string(),
            kind: Some("xs:string".to_string()),
            refers: None,
            min_occurences: None,
            max_occurences: None,
            complex_type: None,
            simple_type: None,
            annotation: None,
          },
          Element {
            name: "country".to_string(),
            kind: Some("xs:string".to_string()),
            refers: None,
            min_occurences: None,
            max_occurences: None,
            complex_type: None,
            simple_type: None,
            annotation: None,
          },
        ],
      }],
      group: None,
    };

    let st = ComplexType {
      name: "fullpersoninfo".to_string(),
      annotation: None,
      attributes: vec![],
      sequence: None,
      simple_content: None,
      complex_content: Some(ComplexContent {
        extension: Some(extension),
      }),
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    //let implementation = extension.implement(&TokenStream::new(), &None, &context);
    let implementation = st.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      "
        # [derive (Clone , Debug , Default , PartialEq , yaserde_derive :: YaDeserialize , yaserde_derive :: YaSerialize)]
        pub struct Fullpersoninfo {
          # [yaserde (flatten)]
          pub base : Personinfo ,
          # [yaserde (rename = \"address\")]
          pub address : String ,
          # [yaserde (rename = \"city\")]
          pub city : String ,
          # [yaserde (rename = \"country\")]
          pub country : String ,
        }
      ",
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }
}
