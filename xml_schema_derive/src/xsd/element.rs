use crate::xsd::{
  annotation::Annotation, complex_type::ComplexType, max_occurences::MaxOccurences,
  rust_types_mapping::RustTypesMapping, XsdContext,
};
use heck::{CamelCase, SnakeCase};
use log::{debug, info};
use proc_macro2::{Span, TokenStream};
use std::io::prelude::*;
use syn::Ident;
use yaserde::YaDeserialize;

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
  pub complex_type: Vec<ComplexType>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}

impl Element {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let struct_name = Ident::new(
      &self.name.replace(".", "_").to_camel_case(),
      Span::call_site(),
    );

    let fields = if let Some(kind) = &self.kind {
      let subtype_mode = if RustTypesMapping::is_xs_string(context, kind) {
        quote!(text)
      } else {
        quote!(flatten)
      };

      let extern_type = RustTypesMapping::get(context, kind);

      quote!(
        #[yaserde(#subtype_mode)]
        pub content: #extern_type,
      )
    } else {
      self
        .complex_type
        .iter()
        .map(|complex_type| complex_type.get_field_implementation(prefix, context))
        .collect()
    };

    let docs = self
      .annotation
      .as_ref()
      .map(|annotation| annotation.get_implementation(&namespace_definition, prefix, context))
      .unwrap_or_else(|| quote!());

    quote! {
      #docs
      #[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }
    }
  }

  pub fn get_subtypes_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.complex_type.is_empty() {
      return quote!();
    }

    self.get_implementation(namespace_definition, prefix, context)
  }

  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
    multiple: bool,
  ) -> TokenStream {
    if self.name == "" {
      return quote!();
    }

    // if self.kind == "md:CompObjEntry-type" {
    //   return quote!();
    // }

    let name = if self.name.to_lowercase() == "type" {
      "Kind".to_string()
    } else {
      self.name.to_snake_case()
    };

    info!("Generate element {:?}", name);

    let name = if multiple { format!("{}s", name) } else { name };

    let attribute_name = Ident::new(&name, Span::call_site());
    let yaserde_rename = &self.name;

    let rust_type = if self.complex_type.is_empty() {
      if let Some(kind) = &self.kind {
        RustTypesMapping::get(context, kind)
      } else {
        panic!(
          "[Element] {} unimplemented type: {:?}",
          self.name, self.kind,
        );
      }
    } else if self.complex_type.first().unwrap().sequence.is_some() {
      let list_wrapper = Ident::new(&self.name, Span::call_site());
      quote!(#list_wrapper)
    } else if self.complex_type.first().unwrap().simple_content.is_some() {
      quote!(String)
    } else {
        println!("{:?}", self);
      panic!(
        "[Element] {} unimplemented complex type with type: {:?}",
        self.name, self.kind
      );
    };

    let rust_type = if multiple {
      quote!(Vec<#rust_type>)
    } else {
      rust_type
    };

    let prefix_attribute = if let Some(prefix) = prefix {
      quote!(, prefix=#prefix)
    } else {
      quote!()
    };

    quote! {
      #[yaserde(rename=#yaserde_rename #prefix_attribute)]
      pub #attribute_name: #rust_type,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static DERIVES: &str =
    "# [ derive ( Clone , Debug , Default , PartialEq , YaDeserialize , YaSerialize ) ] ";

  static DOCS: &str = r#"# [ doc = "Loudness measured in Decibels" ] "#;

  #[test]
  fn extern_type() {
    let element = Element {
      name: "volume".to_string(),
      kind: Some("books:volume-type".to_string()),
      refers: None,
      min_occurences: None,
      max_occurences: None,
      complex_type: vec![],
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: vec!["Loudness measured in Decibels".to_string()],
      }),
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let ts = element.get_implementation(&quote!(), &None, &context);

    assert_eq!(
      ts.to_string(),
      format!(
        "{}{}pub struct Volume {{ # [ yaserde ( flatten ) ] pub content : VolumeType , }}",
        DOCS, DERIVES
      )
    );
  }

  #[test]
  fn xs_string_element() {
    let element = Element {
      name: "volume".to_string(),
      kind: Some("xs:string".to_string()),
      refers: None,
      min_occurences: None,
      max_occurences: None,
      complex_type: vec![],
      annotation: Some(Annotation {
        id: None,
        attributes: vec![],
        documentation: vec!["Loudness measured in Decibels".to_string()],
      }),
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let ts = element.get_implementation(&quote!(), &None, &context);

    assert_eq!(
      ts.to_string(),
      format!(
        "{}{}pub struct Volume {{ # [ yaserde ( text ) ] pub content : String , }}",
        DOCS, DERIVES
      )
    );
  }
}
