use std::fs;

use ahqstore_types::AHQStoreApplication;
use serde_json::{from_str, to_string_pretty};

use super::shared::{gh::has_org_members, user};

use crate::Data;

pub mod android;

pub fn run() {
  let val = fs::read_to_string("./bytes.txt").unwrap();

  let [gh_author, val] = val.split("&").collect::<Vec<_>>()[..] else {
    panic!("Something went wrong");
  };

  let val = Data::from_bytes(&val);

  let app = val.0;

  let resp = app.validate().unwrap_or_else(|x| x);

  println!("{}", &resp);

  if resp.contains("❌") {
    panic!("App Validation Failed!");
  }

  let app_txt = to_string_pretty(&app).unwrap();
  let author = &app.repo.author;

  let author_id = &app.authorId;

  let Some(user) = user::get_author(author_id) else {
    println!("❌ Author {} not found!", &author_id);
    panic!("Author: {} not found!", &author_id);
  };

  if user.github != gh_author && has_org_members(gh_author, &user.github) {
    panic!("Author is not the same as GitHub repo author or GitHub org public member");
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
