mod parser;

use parser::HashMapify;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro]
pub fn hash_mapify(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as HashMapify)
  .to_token_stream()
  .into()
}
