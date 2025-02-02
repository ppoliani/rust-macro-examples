mod parser;

use parser::log_duration_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, input: TokenStream) -> TokenStream {
  log_duration_impl(args, input)
}
