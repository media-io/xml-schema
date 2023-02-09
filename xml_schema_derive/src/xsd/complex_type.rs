use crate::xsd::{
  annotation::Annotation, attribute::Attribute, choice::Choice, complex_content::ComplexContent,
  sequence::Sequence, simple_content::SimpleContent, Implementation, XsdContext,
};
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "complexType"
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexType {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "sequence")]
  pub sequence: Option<Sequence>,
  #[yaserde(rename = "simpleContent")]
  pub simple_content: Option<SimpleContent>,
  #[yaserde(rename = "complexContent")]
  pub complex_content: Option<ComplexContent>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "choice")]
  pub choice: Option<Choice>,
}

impl Implementation for ComplexType {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let struct_name = Ident::new(
      &self.name.replace('.', "_").to_camel_case(),
      Span::call_site(),
    );
    log::info!("Generate sequence");
    let sequence = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.implement(namespace_definition, prefix, context))
      .unwrap_or_else(TokenStream::new);

    log::info!("Generate simple content");
    let simple_content = self
      .simple_content
      .as_ref()
      .map(|simple_content| simple_content.implement(namespace_definition, prefix, context))
      .unwrap_or_else(TokenStream::new);

    let complex_content = self
      .complex_content
      .as_ref()
      .map(|complex_content| {
        let complex_content_type = complex_content.get_field_implementation(context, prefix);
        quote!(
          #[yaserde(flatten)]
          #complex_content_type,
        )
      })
      .unwrap_or_else(TokenStream::new);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.implement(namespace_definition, prefix, context))
      .collect();

    let sequence_sub_types = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_sub_types_implementation(context, namespace_definition, prefix))
      .unwrap_or_else(TokenStream::new);

    let docs = self
      .annotation
      .as_ref()
      .map(|annotation| annotation.implement(namespace_definition, prefix, context))
      .unwrap_or_else(TokenStream::new);

    let choice_sub_types = self
      .choice
      .as_ref()
      .map(|choice| choice.get_sub_types_implementation(context, &namespace_definition, prefix))
      .unwrap_or_else(TokenStream::new);

    let choice_field = self
      .choice
      .as_ref()
      .map(|choice| choice.get_field_implementation(context, prefix))
      .unwrap_or_else(TokenStream::new);

    quote! {
      #docs

      #[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #sequence
        #simple_content
        #complex_content
        #choice_field
        #attributes
      }

      #sequence_sub_types
      #choice_sub_types
    }
  }
}

impl ComplexType {
  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    if self.sequence.is_some() {
      self
        .sequence
        .as_ref()
        .map(|sequence| sequence.get_field_implementation(context, prefix))
        .unwrap_or_else(TokenStream::new)
    } else if self.simple_content.is_some() {
      self
        .simple_content
        .as_ref()
        .map(|simple_content| simple_content.get_field_implementation(context, prefix))
        .unwrap_or_else(TokenStream::new)
    } else if self.choice.is_some() {
      self
        .choice
        .as_ref()
        .map(|choice| choice.get_field_implementation(context, prefix))
        .unwrap_or_else(TokenStream::new)
    } else {
      TokenStream::new()
    }
  }

  pub fn get_integrated_implementation(&self, parent_name: &str) -> TokenStream {
    if self.simple_content.is_some() {
      return quote!(String);
    }

    if self.sequence.is_some() {
      let list_wrapper = Ident::new(parent_name, Span::call_site());
      return quote!(#list_wrapper);
    }

    quote!(String)
  }
}
