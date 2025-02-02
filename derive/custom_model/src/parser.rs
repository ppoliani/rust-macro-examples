use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream, Result as ParseResult}, DeriveInput, Field, Ident, ItemStruct, Path};
use crate::custom_model::{CustomModel, CustomModelArgs};

pub struct DeriveCustomModel {
  custom_model_args: CustomModelArgs,
  item_struct: ItemStruct,
}

impl Parse for DeriveCustomModel {
  fn parse(input: ParseStream) -> ParseResult<Self> {
    let custom_model_args = match CustomModelArgs::from_derive_input(&<DeriveInput as Parse>::parse(input)?) {
      Ok(v) => v,
      Err(err) => {
        // If darling returned an error, generate a token stream from it so that the compiler
        // shows the error in the right location.
        return Err(syn::Error::from(err));
      }
    };

    Ok(Self {
      custom_model_args,
      item_struct: <ItemStruct as Parse>::parse(input)?,
    })
  }
}

impl DeriveCustomModel {
  fn generate(&self) -> TokenStream {
    let CustomModelArgs { models } = &self.custom_model_args;
    let mut output = quote! {};

    // panic if no models are defined
    if models.is_empty() {
      panic!("No models defined for derive_custom_model");
    }

    for model in models {
      let generated_model = self.generate_custom_model(model);
      output.extend(quote! {#generated_model});
    }

    todo!()
  }

  fn generate_custom_model(&self, model: &CustomModel) -> TokenStream {
    let CustomModel {name, fields: target_fields, extra_derives} = model;
    let mut new_fields = quote! {};

    for field in &self.item_struct.fields {
      let Field {
        // The identifier for this field
        ident,
        // Any attributes applied to this field
        attrs,
        // The visibility specifier for this field
        vis,
        // The colon token `:`
        colon_token,
        // The type of this field
        ty,
        ..
      } = field;

      // Make sure that field has an identifier, panic otherwise
      let Some(ident) = ident else {
        panic!("Failed to get struct field identifier")
      };

      // Try to convert field identifier to `Path` which is a type provided
      // by `syn`. We do this because `darling`'s PathList type is just a
      // collection of this type with additional methods on it.
      let path = match Path::from_string(&ident.clone().to_string()) {
        Ok(path) => path,
        Err(error) => panic!("Failed to convert field identifier to path: {error:?}"),
      };

      // skip if the original struct does not have the given requested field name
      if !target_fields.contains(&path) {
        continue;
      }

      // add the field to the output
      new_fields.extend(quote! {
        // any attribute applied to this field
        #(#attrs)*
        #vis #ident #colon_token #ty
      });
    }

    let new_struct = match Ident::from_string(name) {
      Ok(ident) => ident,
      Err(error) => panic!("{error:?}"),
    };

    let mut extra_derives_output = quote! {};
    if !extra_derives.is_empty() {
      extra_derives_output.extend(quote! {
        #(#extra_derives,)*
      });
    }

    quote! {
      #[derive(#extra_derives_output)]
      pub struct #new_struct {
        #new_fields
      }
    }
  }
} 

impl From<&DeriveCustomModel> for TokenStream {
  fn from(parser: &DeriveCustomModel) -> Self {
    parser.generate()
  }
}

impl ToTokens for DeriveCustomModel {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend::<TokenStream>(self.into());
  }
}
