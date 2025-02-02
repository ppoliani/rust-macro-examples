mod parser;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use parser::InitStringHashMapParser;

#[proc_macro_derive(IntoStringHashMap)]
pub fn derive_into_hash_map(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as InitStringHashMapParser)
  .to_token_stream()
  .into()
}
