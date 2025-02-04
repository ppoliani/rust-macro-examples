use syn::{punctuated::Punctuated, token::Comma, GenericArgument, Ident, PathArguments, Type};

pub trait TypeExt {
  fn has_ident(&self) -> bool;
  fn get_ident(&self) -> Option<Ident>;
  fn has_angle_bracketed_generic_args(&self) -> bool;
  fn get_angle_bracketed_generic_args(&self) -> Option<Punctuated<GenericArgument, Comma>>;
  fn get_angle_bracketed_generic_args_idents(&self) -> Option<Vec<Ident>>;
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

    type_path.path.get_ident().map(|i| i.clone())
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

  fn get_angle_bracketed_generic_args_idents(&self) -> Option<Vec<Ident>> {
    let Some(generic_args) = self.get_angle_bracketed_generic_args() else {
      return None
    };

    let mut idents = Vec::new();

    for generic in generic_args {
      let GenericArgument::Type(generic_ty) = generic else {
        continue;
      };

      let Some(ident) = generic_ty.get_ident() else {
        continue;
      };

      idents.push(ident);
    }

    Some(idents)
  }
}
