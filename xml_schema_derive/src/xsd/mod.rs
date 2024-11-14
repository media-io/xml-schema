mod annotation;
mod attribute;
mod complex_content;
mod complex_type;
mod element;
mod extension;
mod group;
mod list;
mod restriction;
mod rust_types_mapping;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;

use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use syn::Visibility;

use xml_schema::{Xsd, XsdContext};

trait Implementation {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
  ) -> TokenStream {
    unimplemented!()
  }

  fn implement_childs(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
    _struct_name: &Ident,
  ) -> TokenStream {
    unimplemented!()
  }

  fn get_type_implementation(
    &self,
    _context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    unimplemented!()
  }

  fn get_subtypes_implementation(
    &self,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    _context: &XsdContext,
  ) -> TokenStream {
    unimplemented!()
  }

  fn get_sub_types_implementation(
    &self,
    _context: &XsdContext,
    _namespace_definition: &TokenStream,
    _prefix: &Option<String>,
  ) -> TokenStream {
    unimplemented!()
  }

  fn get_field_implementation(
    &self,
    _context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    unimplemented!()
  }

  fn get_integrated_implementation(&self, _parent_name: &str) -> TokenStream {
    unimplemented!()
  }
}

pub fn implement_xsd(
  xsd: &Xsd,
  visibility: Visibility,
  target_prefix: &Option<String>,
) -> TokenStream {
  let schema = xsd
    .schema
    .implement(&TokenStream::new(), target_prefix, &xsd.context);

  let mod_name = format_ident!("{}", xsd.name.to_snake_case());

  quote! {
      mod #mod_name {
          #schema
      }

      #visibility use #mod_name::*;
  }
}
