use ahqstore_types::AHQStoreApplication;
use inquire::{Confirm, Text};
use serde_json::from_str;
use std::fs;

#[cfg(feature = "parse")]
mod parser;
mod types;

#[cfg(feature = "parse")]
use parser::*;
use types::*;

macro_rules! ask {
    ($val:ident: bool, $($x:tt)*) => {
        let $val = Confirm::new($($x)*).with_default(true).prompt().unwrap();
        println!("");
    };

    ($val:ident: String, $($x:tt)*) => {
        let $val = Text::new($($x)*).prompt().unwrap();
        println!("");
    }
}

macro_rules! pan {
  ($($x:tt)*) => {
    println!($($x)*);
    std::process::exit(1);
  };
}

fn main() {
  #[cfg(feature = "parse")]
  parser();

  #[cfg(feature = "gen")]
  run();
}

fn run() {
  let _ = fs::create_dir_all("./request");
  let _ = fs::write("./request/app.json", "{}");

  ask!(val: bool, "Fill ./request/app.json with the contents of your app manifest");

  if !val {
    pan!("You write write \"Y\"");
  }

  let store: AHQStoreApplication =
    from_str(&fs::read_to_string("./request/app.json").unwrap()).unwrap();

  let bytes = Data(store).to_bytes();

  fs::write("./bytes.txt", &bytes).unwrap();
  fs::remove_dir_all("./request").unwrap();
}

fn gen_appid() -> String {
  let mut string = String::new();

  string
}
