use std::fs;

use serde_json::to_string_pretty;

use crate::Data;

pub fn run() {
  let val = fs::read_to_string("./bytes.txt").unwrap();

  let [gh_author, val] = val.split("&").collect::<Vec<_>>()[..] else {
    panic!("Something went wrong");
  };

  let val = Data::from_bytes(&val);

  let app = val.0;

  let app_txt = to_string_pretty(&app).unwrap();
  let author = &app.repo.author;

  if &gh_author != &author {
    panic!("Author Mismatch");
  }

  let ltr = author.split_at(1).0;
  let _ = fs::create_dir_all(format!("./manifests/{}/{}", &ltr, &author));
  let _ = fs::write(format!("./manifests/{}/ignore", &ltr), "ignore this file");

  let _ = fs::write(
    format!("./manifests/{}/{}/{}.json", &ltr, &author, &app.appId),
    app_txt,
  );

  println!("Successful");
}
