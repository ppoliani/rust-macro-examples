mod parser;
mod custom_model;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use parser::DeriveCustomModel;

/// Derive macro that helps you generate derived structs from your original struct.
#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as DeriveCustomModel)
  .to_token_stream()
  .into()
}
