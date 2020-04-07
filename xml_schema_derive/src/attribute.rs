use proc_macro2::token_stream::IntoIter;
use proc_macro2::Delimiter;
use proc_macro2::TokenTree;
use syn::Attribute;

#[derive(Debug, PartialEq, Clone)]
pub struct XmlSchemaAttribute {
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
      source: source.unwrap(),
      store_generated_code,
      target_prefix,
    }
  }
}
