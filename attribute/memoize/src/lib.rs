mod parser;

use parser::memoize_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn memoize(args: TokenStream, input: TokenStream) -> TokenStream {
  memoize_impl(args, input)
}
