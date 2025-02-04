use syn::{punctuated::Punctuated, token::Comma, GenericArgument, Ident, PathArguments, Type};

pub trait TypeExt {
  fn has_ident(&self) -> bool;
  fn get_ident(&self) -> Option<Ident>;
  fn has_angle_bracketed_generic_args(&self) -> bool;
  fn get_angle_bracketed_generic_args(&self) -> Option<Punctuated<GenericArgument, Comma>>;
}

impl TypeExt for Type {
  fn has_ident(&self) -> bool {
    let Type::Path(type_path) = self else {
      return false
    };

    type_path.path.segments.first().is_some()
  }

  fn get_ident(&self) -> Option<Ident> {
    let Type::Path(type_path) = self else {
      return None
    };

    type_path.path.segments
    .first()
    .map(|s| s.ident.clone())
  }
  
  fn has_angle_bracketed_generic_args(&self) -> bool {
    let Some(generic_args) = self.get_angle_bracketed_generic_args() else {
      return false
    };

    generic_args.len() > 0
  }

  fn get_angle_bracketed_generic_args(&self) -> Option<Punctuated<GenericArgument, Comma>> {
    let Type::Path(type_path) = self else {
      return None
    };

    let path_arguments = &type_path.path
    .segments
    .first()
    .unwrap()
    .arguments;

    let PathArguments::AngleBracketed(angle_bracketed) = path_arguments else {
      return None
    };

    Some(angle_bracketed.args.clone())
  }
}
