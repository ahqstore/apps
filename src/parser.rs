use ahqstore_types::AHQStoreApplication;
use serde_json::from_str;
use std::{
  fs::{self, File},
  io::Write,
};

struct Map {
  entries: usize,
  files: usize,
  c_file: File,
  search: File,
}

impl Map {
  fn new() -> Self {
    let _ = fs::create_dir_all("./db/map");
    let _ = fs::create_dir_all("./db/search");
    let _ = fs::create_dir_all("./db/apps");
    let _ = fs::create_dir_all("./db/dev");
    let _ = fs::create_dir_all("./db/res");

    let mut file = File::create("./db/map/1.json").unwrap();
    let _ = file.write(b"{");

    let mut search = File::create("./db/search/1.json").unwrap();
    let _ = search.write(b"[");

    Self {
      entries: 0,
      files: 1,
      c_file: file,
      search,
    }
  }

  fn close_file(&mut self) {
    let _ = self.search.write_all(b"]");
    let _ = self.search.flush();
    let _ = self.c_file.write_all(b"}");
    let _ = self.c_file.flush();
  }

  fn new_file(&mut self) {
    self.files += 1;
    self.entries = 0;
    self.close_file();

    let mut map = File::create("./db/map/1.json").unwrap();
    let _ = map.write(b"{");

    let mut search = File::create("./db/map/1.json").unwrap();
    let _ = search.write(b"[");

    self.c_file = map;
    self.search = search;
  }

  fn add_author(&mut self, author: &str, app_id: &str) {
    let file = format!("./db/dev/{}", author);
    let mut val = fs::read_to_string(&file).unwrap_or("".to_string());
    val.push_str(&format!("{}\n", &app_id));

    let _ = fs::write(&file, val);
  }

  fn add(&mut self, app: AHQStoreApplication) {
    if self.entries >= 100_000 {
      self.new_file();
    }
    println!("{}", self.entries);
    if self.entries > 0 {
      let _ = self.c_file.write(b",");
      let _ = self.search.write(b",");
    }

    self.add_author(&app.authorId, &app.appId);
    self.entries += 1;

    let _ = self
      .c_file
      .write(format!("\"{}\":\"{}\"", app.appDisplayName, app.appId).as_bytes());
    let _ = self.search.write(
      format!(
        "{{\"name:\": {:?}, \"title\": {:?}, \"id\": {:?}}}",
        app.appDisplayName, app.appShortcutName, app.appId
      )
      .as_bytes(),
    );

    let (app_str, res) = app.export();

    let _ = fs::write(
      format!("./db/apps/{}.json", &app.appId),
      app_str,
    );

    for (id, bytes) in res {
      let _ = fs::write(
        format!("./db/res/{}/{}", &app.appId, id),
        bytes
      );
    }
  }

  fn finish(mut self) {
    self.close_file();

    let _ = fs::write("./db/total", self.files.to_string());
  }
}

pub fn parser() {
  println!("⏲️ Please wait...");
  let _ = fs::remove_dir_all("./db");
  let _ = fs::create_dir_all("./db");

  let _ = fs::copy("./manifests/home.json", "./db/home.json");

  let mut map = Map::new();

  for letter in fs::read_dir("./manifests").unwrap() {
    let ltr = letter.unwrap();

    if ltr.file_name().to_string_lossy() != "home.json" {
      for user in fs::read_dir(ltr.path()).unwrap() {
        let usr = user.unwrap();

        let name = usr.file_name();
        let name = name.to_string_lossy();
        if &name != "ignore" {
          for repo in fs::read_dir(usr.path()).unwrap() {
            let repo = repo.unwrap();

            let id = repo.file_name();
            let id = id.to_string_lossy();

            if &id != "user" {
              let data = fs::read_to_string(repo.path()).unwrap();
              let data: AHQStoreApplication = from_str(&data).unwrap();

              println!(
                "🔎 {}->{}->{}",
                name.split_at(1).0,
                name,
                &data.appDisplayName
              );
              map.add(data);
            }
          }
        }
      }
    }
  }
  map.finish();
  println!("✅ Done!");
}
