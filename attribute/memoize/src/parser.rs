use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ItemFn};
use quote::quote;

#[derive(FromMeta)]
struct CachedParams {
  // Accept any expression that we should use to compute the
  // key. This can be a constant string, or some computation
  // based on function arguments.
  keygen: Option<Expr>,
}

pub(crate) fn memoize_impl(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_args = match NestedMeta::parse_meta_list(args.into()) {
    Ok(v) => v,
    Err(e) => return TokenStream::from(Error::from(e).write_errors())
  };

  let CachedParams {keygen} = match CachedParams::from_list(&attr_args) {
    Ok(v) => v,
    Err(e) => return TokenStream::from(Error::from(e).write_errors()),
  };

  // Parse the input target item as a function
  let ItemFn {
    // The function signature
    sig,
    // The visibility specifier of this function
    vis,
    // The function block or body
    block,
    // Other attributes applied to this function
    attrs,
  } = parse_macro_input!(input as ItemFn);

  // Generate our key statement based on given param (or lack thereof)
  let key_statement = if let Some(keygen) = keygen {
    quote! {
      let __cache_key = #keygen;
    }
  } else {
    // If no `keygen` was provided, use the name of the function as cache key.
    let fn_name = sig.ident.to_string();
    quote! {
      let __cache_key = #fn_name;
    }
  };

  quote! {
    use std::{collections::HashMap, sync::LazyLock, str::from_utf8, sync::RwLock};

    static CACHE: LazyLock<RwLock<HashMap<String, Vec<u8>>>> = LazyLock::new(|| {
      RwLock::new(HashMap::new())
    });

    #(#attrs)*
    #vis #sig {
      // we need to add __cache_key to local scope
      #key_statement

      let mut cache = CACHE.write().unwrap();
      let Some(value) = cache.get(&__cache_key) else {
        println!("Data is not fetched from cached");
        let output = #block;
        cache.insert(__cache_key, output.as_bytes().to_vec());
        
        return output
      };

      println!("Data is fetched from cached");
      from_utf8(value).unwrap().to_string()
    }
  }.into()
}
