use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream}, Data, DataEnum, DataStruct, DeriveInput, Field, Generics, Ident, Result as ParseResult, Variant
};

enum FieldType {
  Struct(Vec<Field>),
  Enum(Vec<Variant>),
}

impl FieldType {
  fn get_struct_description(fields: &Vec<Field>) -> String {
    format!(
      "A struct with the following fields \n: {}",
      quote! {
        #(#fields.ident: #fields.ty),*
      }
    )
  }

  fn get_enum_description(variants: &Vec<Variant>) -> String {
    format!(
      "An enum with the following variants \n: {}",
      quote! {
        #(#variants.ident: #variants.fields.ident: #variants.fields.ty),*
      }
    )
  }
}

pub struct Describe {
  field_ty: FieldType,
  ident: Ident,
  generics: Generics,
}

impl Describe {
  fn get_description_for_struct(data: DataStruct) -> FieldType {
    let fields = data.fields.iter().map(|f| f.clone()).collect::<Vec<_>>();
    FieldType::Struct(fields)
  }

  fn get_description_for_enum(data: DataEnum) -> FieldType {
    let variants = data.variants.iter().map(|v| v.clone()).collect::<Vec<_>>();
    FieldType::Enum(variants)
  } 
}

impl Parse for Describe {
  fn parse(input: ParseStream) -> ParseResult<Self> {
    let derive_input = <DeriveInput as Parse>::parse(input)?;
    let field_ty = match derive_input.data {
      Data::Struct(data_struct) => Self::get_description_for_struct(data_struct),
      Data::Enum(data_enum) => Self::get_description_for_enum(data_enum),
      _ => panic!("Unions not supported"),
    };
    
    Ok(Describe {
      field_ty,
      ident: derive_input.ident,
      generics: derive_input.generics,
    })
  }
}

impl ToTokens for Describe {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let generics = &self.generics;
    let ident = &self.ident;
    let field_ty = &self.field_ty;

    let output = quote! {
      impl #generics #ident #generics {
        fn describe(&self) -> String {
          #field_ty
        }
      }
    };

    tokens.extend::<TokenStream>(output);
  }
}

impl ToTokens for FieldType {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let descr = match self {
      FieldType::Struct(fields) => Self::get_struct_description(fields),
      FieldType::Enum(variants) => Self::get_enum_description(variants),
    };
    let output = quote! {#descr};

    tokens.extend::<TokenStream>(output);
  }
}
