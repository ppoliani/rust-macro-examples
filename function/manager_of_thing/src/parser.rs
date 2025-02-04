use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::{parse::{Parse, ParseStream}, Type, Ident};
use macro_utils::type_ext::TypeExt;

/// 
/// ```
/// fn_macro_custom_syntax! {
///   ThingManager<K, V>
///   where K: Send + Sync + Default + 'static, V: Send + Sync + Default + 'static
///   for std::collections::HashMap<K, V>
/// }
/// ```
/// Converts into
/// ```
/// /// Generated manager ThingManager.
/// struct ThingManager<K, V>
/// where
///     K: Send + Sync + Default + 'static,
///     V: Send + Sync + Default + 'static,
/// {
///     wrapped_thing: std::collections::HashMap<K, V>,
/// }
/// ```
pub struct ManagerOfThing {
  manager_name_ident: Ident,
  manager_ty: Type,
}

impl Parse for ManagerOfThing {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let manager_ty = input.parse::<Type>()?;
    let manager_name_ident = manager_ty.get_ident();
    todo!()
  }
}

impl ToTokens for ManagerOfThing {
  fn to_tokens(&self, tokens: &mut TokenStream) { 
    let output = quote! {};
    tokens.extend::<TokenStream>(output);
  }
}
