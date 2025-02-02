use custom_model::CustomModel;
use init_string_hash_map::IntoStringHashMap;

#[derive(CustomModel)]
#[custom_model(model(
  name = "Username",
  fields(username, first_name),
  extra_derives(Debug, Clone, IntoStringHashMap)
))]
pub struct User {
  pub username: String,
  pub first_name: String,
  pub last_name: String,
  pub age: u32,
}

fn main() {
  let username = Username {
    username: "ppoliani".to_string(),
    first_name: "Pavlos".to_string(),
  };

  let username_map: HashMap<String, String> = username.into();
  println!("Username Map {:?}", username_map);
}
