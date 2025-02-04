use std::collections::HashMap;

use manager_of_thing::manager_of_thing;

manager_of_thing! {
  ThingManager<K, V>
  where String: Send + Sync + Default + 'static, V: Send + Sync + Default + 'static
  for std::collections::HashMap<K, V>
}

fn main() {
  let thing = ThingManager {
    wrapped_thing: HashMap::<String, u8>::new(),
  };

  dbg!(thing);
}
