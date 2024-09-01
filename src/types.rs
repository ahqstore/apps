use ahqstore_types::AHQStoreApplication;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data(pub AHQStoreApplication);

impl Data {
  pub fn to_bytes(&self) -> String {
    let stri = to_string(&self.0).unwrap();
    to_string(stri.as_bytes()).unwrap()
  }

  pub fn from_bytes(bytes: &str) -> Self {
    let val: Vec<u8> = from_str(bytes).unwrap();
    let val = String::from_utf8_lossy(&val);

    Self(from_str(&val).unwrap())
  }
}
