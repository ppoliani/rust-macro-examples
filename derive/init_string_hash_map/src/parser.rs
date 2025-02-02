use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream, Result as ParseResult}, ItemStruct};

pub struct InitStringHashMapParser(ItemStruct);

impl InitStringHashMapParser {
  fn generate(&self) -> TokenStream {
    let struct_ident = &self.0.ident;
    let field_idents = self.0.fields.iter()
    .map(|f| f.ident.as_ref().expect("have field"))
    .collect::<Vec<_>>();

    quote! {
      impl From<#struct_ident> for HashMap<String, String> {
        fn from(value: #struct_ident) -> Self {
          let mut hash_map = HashMap::<String, String>::new();

          /// his syntax is what allows you to make use of any iterator inside of the parenthesis,
          /// and it will repeat that block of code for all items in the iterator, while replacing
          /// the variable with correct item in each iteration.
          /// This is similar to doing
          /// 
          /// ```
          /// let mut implementation = quote!{
          ///   let mut hash_map = std::collections::HashMap::<String, String>::new();
          /// };
          /// 
          /// for field in fields {
          ///   let identifier = field.ident.as_ref().unwrap();
          ///   implementation.extend(quote!{
          ///     hash_map.insert(stringify!(#identifier).to_string(), String::from(value.#identifier)); 
          ///   }); 
          /// }
          /// ```
          #(
            hash_map.insert(stringify!(#field_idents), value.#field_idents);
          )*

          hash_map
        }
      }
    }
  }
} 

impl Parse for InitStringHashMapParser {
  fn parse(input: ParseStream) -> ParseResult<Self> {
    let strct = <ItemStruct as Parse>::parse(input)?;
    Ok(Self(strct))
  }
}

impl From<&InitStringHashMapParser> for TokenStream {
  fn from(parser: &InitStringHashMapParser) -> Self {
    parser.generate()
  }
}

impl ToTokens for InitStringHashMapParser {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend::<TokenStream>(self.into());
  }
}
