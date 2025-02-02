use proc_macro::TokenStream;

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(_item: TokenStream) -> TokenStream {
  todo!()
}
