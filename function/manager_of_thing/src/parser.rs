use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::{parse::{Parse, ParseStream}, Ident, Token, Type, WhereClause};
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
  where_clause: Option<WhereClause>,
  thing_ty: Type,
}

impl Parse for ManagerOfThing {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let manager_ty = input.parse::<Type>()?;
    
    // Optional where clause, eg: `where K: Send+Sync+'static, V: Send+Sync+'static`.
    let mut where_clause: Option<WhereClause> = None;
    if input.peek(Token![where]) {
      where_clause = Some(input.parse::<WhereClause>()?);
    }

    if let Some(idents) = &manager_ty.get_angle_bracketed_generic_args_idents() {
      let _where = quote! {
        where #(#idents: Send + Sync + 'static)*;
      };
      where_clause = Some(syn::parse(_where.into()).unwrap());
    }

    input.parse::<Token![for]>()?;
    let thing_ty = input.parse::<Type>()?;
    let manager_name_ident = thing_ty.get_ident().expect("Expected Type::Path::TypePath.segments to have an Ident");

    Ok(ManagerOfThing {
      manager_name_ident,
      manager_ty,
      where_clause,
      thing_ty,
    })
  }
}

impl ToTokens for ManagerOfThing {
  fn to_tokens(&self, tokens: &mut TokenStream) { 
    let ManagerOfThing {
      manager_name_ident,
      manager_ty,
      thing_ty,
      where_clause,
    } = self;

    let doc_str = format!("Generated manager {}.", &manager_name_ident);
    let output = quote! {
      #[doc = #doc_str]
      struct #manager_ty #where_clause {
        wrapped_thing: #thing_ty
      }
    };

    tokens.extend::<TokenStream>(output);
  }
}
