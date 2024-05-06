use crate::xsd::{
  annotation::Annotation, complex_type::ComplexType, max_occurences::MaxOccurences,
  rust_types_mapping::RustTypesMapping, simple_type::SimpleType, Implementation, XsdContext,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Element {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename = "type", attribute)]
  pub kind: Option<String>,
  #[yaserde(rename = "ref", attribute)]
  pub refers: Option<String>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Option<ComplexType>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}

impl Implementation for Element {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let struct_name = Ident::new(
      &self.name.replace('.', "_").to_upper_camel_case(),
      Span::call_site(),
    );

    let (fields, extra_structs) = if let Some(kind) = &self.kind {
      let subtype_mode = if RustTypesMapping::is_xs_string(context, kind) {
        quote!(text)
      } else {
        quote!(flatten)
      };

      let extern_type = RustTypesMapping::get(context, kind);

      (
        quote!(
          #[yaserde(#subtype_mode)]
          pub content: xml_schema_types::#extern_type,
        ),
        quote!(),
      )
    } else {
      let fields_definition = self
        .complex_type
        .iter()
        .map(|complex_type| complex_type.get_field_implementation(context, prefix))
        .collect();

      (fields_definition, quote!())
    };

    let docs = self
      .annotation
      .as_ref()
      .map(|annotation| annotation.implement(namespace_definition, prefix, context))
      .unwrap_or_default();

    quote! {
      #docs
      #[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }

      #extra_structs
    }
  }
}

impl Element {
  pub fn get_subtypes_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.complex_type.is_none() {
      return quote!();
    }

    self.implement(namespace_definition, prefix, context)
  }

  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let refers = self.get_refers();
    if self.name.is_empty() && refers.is_none() {
      return quote!();
    }

    let multiple = self.max_occurences.is_some()
      && self.max_occurences != Some(MaxOccurences::Number { value: 1 });

    let name = if self.name.to_lowercase() == "type" {
      "kind".to_string()
    } else if !self.name.is_empty() {
      self.name.to_snake_case()
    } else {
      refers
        .expect("[Element] refers should be defined")
        .to_snake_case()
    };

    log::info!("Generate element {:?}", name);

    let name = if multiple {
      format!("{name}_list")
    } else {
      name
    };

    let attribute_name = Ident::new(&name, Span::call_site());
    let yaserde_rename = if !self.name.is_empty() {
      &self.name
    } else {
      refers.expect("[Element] refers should be defined")
    };

    let rust_type = if let Some(complex_type) = &self.complex_type {
      complex_type.get_integrated_implementation(&self.name)
    } else if let Some(simple_type) = &self.simple_type {
      simple_type.get_type_implementation(context, &Some(self.name.to_owned()))
    } else if let Some(kind) = &self.kind {
      RustTypesMapping::get(context, kind)
    } else if let Some(refers) = refers {
      RustTypesMapping::get(context, refers)
    } else {
      panic!(
        "[Element] {:?} unimplemented type: {:?}",
        self.name, self.kind,
      );
    };

    let module = (!context.is_in_sub_module()
      && !self
        .kind
        .as_ref()
        .map(|kind| {
          RustTypesMapping::is_xs_string(context, kind)
            || RustTypesMapping::is_xs_int(context, kind)
        })
        .unwrap_or_default())
    .then_some(quote!(xml_schema_types::))
    .unwrap_or_default();

    let rust_type = if multiple {
      quote!(Vec<#module#rust_type>)
    } else if self.min_occurences == Some(0) {
      quote!(Option<#module#rust_type>)
    } else {
      quote!(#module#rust_type)
    };

    let prefix_attribute = prefix
      .as_ref()
      .map(|prefix| quote!(, prefix=#prefix))
      .unwrap_or_default();

    quote! {
      #[yaserde(rename=#yaserde_rename #prefix_attribute)]
      pub #attribute_name: #rust_type,
    }
  }

  fn get_refers(&self) -> Option<&str> {
    self.refers.as_ref().and_then(|refers| {
      if refers.is_empty() {
        None
      } else {
        Some(refers.as_str())
      }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  static DERIVES: &str =
    "#[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]";

  static DOCS: &str = r#"#[doc = "Loudness measured in Decibels"]"#;

  #[test]
  fn extern_type() {
    let element = Element {
      name: "volume".to_string(),
      kind: Some("books:volume-type".to_string()),
      refers: None,
      min_occurences: None,
      max_occurences: None,
      complex_type: None,
      simple_type: None,
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: vec!["Loudness measured in Decibels".to_string()],
      }),
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = element.implement(&quote!(), &None, &context);

    let expected = TokenStream::from_str(&format!(
      r#"
        {DOCS}
        {DERIVES}
        pub struct Volume {{
          #[yaserde(flatten)]
          pub content: xml_schema_types::VolumeType,
        }}"#
    ))
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn xs_string_element() {
    let element = Element {
      name: "volume".to_string(),
      kind: Some("xs:string".to_string()),
      refers: None,
      min_occurences: None,
      max_occurences: None,
      complex_type: None,
      simple_type: None,
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: vec!["Loudness measured in Decibels".to_string()],
      }),
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = element.implement(&quote!(), &None, &context);

    let expected = TokenStream::from_str(&format!(
      r#"
        {DOCS}
        {DERIVES}
        pub struct Volume {{
          #[yaserde(text)]
          pub content: xml_schema_types::String,
        }}"#
    ))
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn refers_element_field_implementation() {
    // <xs:element ref="OwnedType" />
    let element = Element {
      name: "".to_string(),
      kind: None,
      refers: Some("OwnedType".to_string()),
      min_occurences: None,
      max_occurences: None,
      complex_type: None,
      simple_type: None,
      annotation: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = element.get_field_implementation(&context, &None);

    let expected = TokenStream::from_str(
      r#"#[yaserde(rename = "OwnedType")] pub owned_type : xml_schema_types :: OwnedType ,"#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());

    // <xs:element ref="OwnedType"  minOccurs="0" maxOccurs="unbounded" />
    let element = Element {
      name: "".to_string(),
      kind: None,
      refers: Some("OwnedType".to_string()),
      min_occurences: Some(0),
      max_occurences: Some(MaxOccurences::Unbounded),
      complex_type: None,
      simple_type: None,
      annotation: None,
    };

    let implementation = element.get_field_implementation(&context, &None);

    let expected = TokenStream::from_str(
      r#"#[yaserde(rename = "OwnedType")] pub owned_type_list : Vec < xml_schema_types :: OwnedType > ,"#
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }
}
