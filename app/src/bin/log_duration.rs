use log_duration::log_duration;

#[log_duration]
fn func() -> String {
  for _ in 0..100000 {}
  "Success".to_string()
}

fn main() {
  func();
}
