// Adds YaSerialize and YaDeserialize implementations for types that support FromStr and Display traits.
// Code originally from `xsd-parser-rs`

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn primitive_yaserde(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);

  let struct_name = &ast.ident;
  let struct_name_literal = &ast.ident.to_string();

  let serde = quote! {
      impl ::yaserde::YaSerialize for #struct_name {
          fn name() -> &'static str {
              #struct_name_literal
          }
          fn serialize<W: ::std::io::Write>(
              &self,
              writer: &mut ::yaserde::ser::Serializer<W>,
          ) -> ::std::result::Result<(), ::std::string::String> {
            ::yaserde::primitives::serialize_primitives(
                  self,
                  #struct_name_literal,
                  writer, |s| s.to_string(),
              )
          }

          fn serialize_attributes(
              &self,
              attributes: ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
              namespace: ::xml::namespace::Namespace,
          ) -> ::std::result::Result<
              (
                  ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
                  ::xml::namespace::Namespace,
              ),
              ::std::string::String,
          > {
              Ok((attributes, namespace))
          }
      }

      impl ::yaserde::YaDeserialize for #struct_name {
          fn deserialize<R: ::std::io::Read>(
              reader: &mut ::yaserde::de::Deserializer<R>,
          ) -> ::std::result::Result<Self, ::std::string::String> {
              ::yaserde::primitives::deserialize_primitives(
                  reader,
                  |s| #struct_name::from_str(s).map_err(|e| e.to_string()),
              )
          }
      }
  };

  serde.into()
}

pub fn hexbinary_serde(input: TokenStream) -> TokenStream {
  let first = input.clone();
  let DeriveInput { ident, .. } = parse_macro_input!(first);
  quote! {
    impl std::fmt::Display for #ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
      }
    }

    impl ::std::str::FromStr for #ident {
      type Err = ::std::string::String;

      fn from_str(s: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        bitflags::parser::from_str(s).map_err(|e| e.to_string())
      }
    }
  }
  .into()
}

pub fn primitive_serde(input: TokenStream) -> TokenStream {
  let first = input.clone();
  let DeriveInput { ident, .. } = parse_macro_input!(first);
  quote! {
    impl std::fmt::Display for #ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
      }
    }

    impl ::std::str::FromStr for #ident {
      type Err = ::std::string::String;

      fn from_str(s: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        s.parse::<Self>().map_err(|e| e.to_string())
      }
    }
  }
  .into()
}
