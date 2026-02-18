use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("Kinan", "Hello");
  map.insert("Zach", "Hi!");
  let my_variable = map.get("Kinan");
  match my_variable{
    Some(value) => println!("the variable says {value} "),
    None => println!("There is nothing here"),
  }
}