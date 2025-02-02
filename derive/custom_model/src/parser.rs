use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream, Result as ParseResult},
  Data, DataStruct, DeriveInput, Field, Ident, Path,
};
use crate::custom_model::{CustomModelDef, CustomModelArgs};

pub struct CustomModel {
  custom_model_args: CustomModelArgs,
  data_struct: DataStruct,
}

impl Parse for CustomModel {
  fn parse(input: ParseStream) -> ParseResult<Self> {
    let derive_input = <DeriveInput as Parse>::parse(input)?;
    let Data::Struct(data_struct) = derive_input.data.clone() else {
      panic!("DeriveCustomModel can only be used with named structs")
    };
    

    let custom_model_args = match CustomModelArgs::from_derive_input(&derive_input) {
      Ok(v) => v,
      Err(err) => {
        // If darling returned an error, generate a token stream from it so that the compiler
        // shows the error in the right location.
        return Err(syn::Error::from(err));
      }
    };

    Ok(Self {
      custom_model_args,
      data_struct,
    })
  }
}

impl CustomModel {
  pub fn generate(&self) -> TokenStream {
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

    output
  }

  fn generate_custom_model(&self, model: &CustomModelDef) -> TokenStream {
    let CustomModelDef {name, fields: target_fields, extra_derives} = model;
    let mut new_fields = quote! {};

    for field in &self.data_struct.fields {
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
        #vis #ident #colon_token #ty,
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

impl From<&CustomModel> for TokenStream {
  fn from(parser: &CustomModel) -> Self {
    parser.generate()
  }
}

impl ToTokens for CustomModel {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend::<TokenStream>(self.into());
  }
}
