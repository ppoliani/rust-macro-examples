mod parser;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use parser::InitStringHashMap;

/// Convert structs into hash maps, that uses the String type for both keys and values.
/// This means that it should work with any struct where all of the fields are convertible
/// to String type using the Into trait.
#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hash_map(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as InitStringHashMap)
  .to_token_stream()
  .into()
}
