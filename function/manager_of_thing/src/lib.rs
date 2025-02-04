mod parser;

use parser::ManagerOfThing;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro]
pub fn manager_of_thing(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as ManagerOfThing)
  .to_token_stream()
  .into()
}
