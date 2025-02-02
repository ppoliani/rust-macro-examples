use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as ItemFn);
  let ItemFn {
    // The function signature
    sig,
    // The visibility specifier of this function
    vis,
    // The function block or body
    block,
    // Other attributes applied to this function
    attrs,
  } = input;

  // Extract statements in the body of the functions
  let statements = block.stmts;
  let fn_ident = &sig.ident;

  quote! {
    use std::time::Instant;

    #(#attrs)*
    #vis #sig {
      let __start = Instant::now();
      let __result = {
        #(#statements)*
      };

      println!("{} took {}μs", stringify!(#fn_ident), __start.elapsed().as_micros());

      __result
    }
  }.into()
}
