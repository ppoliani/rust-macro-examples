use memoize::memoize;

#[memoize(keygen = r#"format!("{first_name} {last_name}")"#)]
fn test_cache(first_name: String, last_name: String) -> String {
  format!("{first_name} {last_name}")
}

fn main() {
  test_cache("John".to_string(), "Appleseed".to_string());
  test_cache("John".to_string(), "Appleseed".to_string());
  test_cache("John".to_string(), "Doe".to_string());
}
