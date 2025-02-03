use syn::{Ident, Type};

pub trait TypeExt {
  fn has_ident(&self) -> bool;
  fn get_ident(&self) -> Option<Ident>;
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
}
