mod parser;

use parser::constant_string_impl;
use proc_macro::TokenStream;

#[proc_macro]
pub fn constant_string(input: TokenStream) -> TokenStream {
  constant_string_impl(input)
}
