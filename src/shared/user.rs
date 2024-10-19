use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub name: String,
  pub id: String,
  pub github: String,
  pub avatar_url: String,
}

pub fn get_author(id: &str) -> Option<User> {
  let data = fs::read_to_string(format!("./users/{id}.json")).ok()?;

  from_str(&data).ok()
}

pub fn set_author(id: &str, user: &User) {
  let _ = fs::write(format!("./users/{id}.json"), to_string(user).unwrap());
}
