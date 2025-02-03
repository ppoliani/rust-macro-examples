use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, parse_str, LitStr};
use quote::quote;

pub(crate) fn constant_string_impl(input: TokenStream) -> TokenStream {
  let const_value = parse_macro_input!(input as LitStr);
  let const_name: Ident = parse_str(&const_value.value()).unwrap();

  quote! {
    pub const #const_name: &str = #const_value;
  }.into()
}
