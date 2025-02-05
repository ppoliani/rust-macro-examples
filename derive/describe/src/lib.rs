mod parser;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use parser::Describe;

/// Convert structs into hash maps, that uses the String type for both keys and values.
/// This means that it should work with any struct where all of the fields are convertible
/// to String type using the Into trait.
#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as Describe)
  .to_token_stream()
  .into()
}
