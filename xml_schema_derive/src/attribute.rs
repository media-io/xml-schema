use proc_macro2::token_stream::IntoIter;
use proc_macro2::Delimiter;
use proc_macro2::TokenTree;
use syn::Attribute;

#[derive(Debug, PartialEq, Clone)]
pub struct XmlSchemaAttribute {
  pub log_level: log::Level,
  pub source: String,
  pub target_prefix: Option<String>,
  pub store_generated_code: Option<String>,
}

fn get_value(iter: &mut IntoIter) -> Option<String> {
  if let (Some(TokenTree::Punct(operator)), Some(TokenTree::Literal(value))) =
    (iter.next(), iter.next())
  {
    if operator.as_char() == '=' {
      Some(value.to_string().replace("\"", ""))
    } else {
      None
    }
  } else {
    None
  }
}

impl XmlSchemaAttribute {
  pub fn parse(attrs: &[Attribute]) -> XmlSchemaAttribute {
    let mut log_level = log::Level::Warn;
    let mut source = None;
    let mut store_generated_code = None;
    let mut target_prefix = None;

    for attr in attrs.iter() {
      let mut attr_iter = attr.clone().tokens.into_iter();
      if let Some(token) = attr_iter.next() {
        if let TokenTree::Group(group) = token {
          if group.delimiter() == Delimiter::Parenthesis {
            let mut attr_iter = group.stream().into_iter();

            while let Some(item) = attr_iter.next() {
              if let TokenTree::Ident(ident) = item {
                match ident.to_string().as_str() {
                  "source" => {
                    source = get_value(&mut attr_iter);
                  }
                  "store_generated_code" => {
                    store_generated_code = get_value(&mut attr_iter);
                  }
                  "target_prefix" => {
                    target_prefix = get_value(&mut attr_iter);
                  }
                  "log_level" => {
                    if let Some(value) = get_value(&mut attr_iter) {
                      log_level = match value.as_ref() {
                        "trace" => log::Level::Trace,
                        "debug" => log::Level::Debug,
                        "info" => log::Level::Info,
                        "warn" => log::Level::Warn,
                        "error" => log::Level::Error,
                        _ => log::Level::Warn,
                      };
                    }
                  }
                  _ => {}
                }
              }
            }
          }
        }
      }
    }

    if source.is_none() {
      panic!("Unable to expand schema, missing source paramater");
    }

    XmlSchemaAttribute {
      log_level,
      source: source.unwrap(),
      store_generated_code,
      target_prefix,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proc_macro2::{Span, TokenStream};
  use std::str::FromStr;
  use syn::{
    punctuated::Punctuated,
    token::{Bracket, Pound},
    AttrStyle::Outer,
    Ident, Path, PathArguments, PathSegment,
  };

  fn generate_attributes(content: &str) -> Vec<Attribute> {
    let mut punctuated = Punctuated::new();
    punctuated.push(PathSegment {
      ident: Ident::new("yaserde", Span::call_site()),
      arguments: PathArguments::None,
    });

    vec![Attribute {
      pound_token: Pound {
        spans: [Span::call_site()],
      },
      style: Outer,
      bracket_token: Bracket {
        span: Span::call_site(),
      },
      path: Path {
        leading_colon: None,
        segments: punctuated,
      },
      tokens: TokenStream::from_str(content).unwrap(),
    }]
  }

  #[test]
  #[should_panic]
  fn parse_empty_attributes() {
    let attributes = vec![];
    XmlSchemaAttribute::parse(&attributes);
  }

  #[test]
  fn parse_source_attribute() {
    let attributes = generate_attributes(r#"(source = "schema.xsd")"#);
    assert_eq!(
      XmlSchemaAttribute {
        log_level: log::Level::Warn,
        source: "schema.xsd".to_string(),
        target_prefix: None,
        store_generated_code: None,
      },
      XmlSchemaAttribute::parse(&attributes)
    );
  }

  #[test]
  fn parse_attributes() {
    let attributes = generate_attributes(
      r#"(source = "schema.xsd", log_level="debug", target_prefix="prefix", store_generated_code="sample.rs")"#,
    );
    assert_eq!(
      XmlSchemaAttribute {
        log_level: log::Level::Debug,
        source: "schema.xsd".to_string(),
        target_prefix: Some("prefix".to_string()),
        store_generated_code: Some("sample.rs".to_string()),
      },
      XmlSchemaAttribute::parse(&attributes)
    );
  }
}
