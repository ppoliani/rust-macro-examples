use init_string_hash_map::IntoStringHashMap;

#[derive(IntoStringHashMap)]
struct User {
  username: String,
  first_name: String,
  last_name: String,
}

fn main() {
  let user = User {
    username: "ppoliani".to_string(),
    first_name: "Pavlos".to_string(),
    last_name: "Polianidis".to_string(),
  };

  let user_map: HashMap<String, String> = user.into();
  println!("User Map {:?}", user_map);
}
