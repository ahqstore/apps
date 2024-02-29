use std::fs;

pub fn remove_manifest() {
  let val = fs::read_to_string("./bytes.txt").unwrap();
  if !fs::metadata(&val).unwrap().is_file() {
    panic!("Expected File!");
  }

  fs::remove_file(val).unwrap();
  println!("Success!");
}
