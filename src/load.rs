use reqwest::blocking::Client;
use std::fs;

use ahqstore_types::AHQStoreApplication;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::Data;

#[derive(Debug, Serialize, Deserialize)]
struct AccData {
  pub linked_acc: Vec<String>,
}

pub fn run() {
  let val = fs::read_to_string("./bytes.txt").unwrap();

  let [gh_author, val] = val.split("&").collect::<Vec<_>>()[..] else {
    panic!("Something went wrong");
  };

  let val = Data::from_bytes(&val);

  let app = val.0;

  let app_txt = to_string_pretty(&app).unwrap();
  let author = &app.repo.author;

  let client = Client::new();
  let val = format!(
    "https://ahqstore-server.onrender.com/users/{}",
    &app.authorId
  );

  let val: AccData = client.get(&val).send().unwrap().json().unwrap();

  if !val.linked_acc.contains(&gh_author.into()) {
    panic!("Account not linked!");
  }

  no_duped_appid(&app);

  let ltr = author.split_at(1).0;
  let _ = fs::create_dir_all(format!("./manifests/{}/{}", &ltr, &author));
  let _ = fs::write(format!("./manifests/{}/ignore", &ltr), "ignore this file");

  let _ = fs::write(
    format!("./manifests/{}/{}/{}.json", &ltr, &author, &app.appId),
    app_txt,
  );

  println!("Successful");
}

fn no_duped_appid(app: &AHQStoreApplication) {
  if let Ok(val) = fs::read_to_string(format!("./db/apps/{}.json", &app.appId)) {
    if let Ok(ap) = from_str::<AHQStoreApplication>(&val) {
      if ap.authorId != app.authorId {
        panic!("AppId already exists for another author!");
      }
    }
  }
}
