use std::process::{Command, Stdio};

fn get_aapt_path() -> String {
  let andy = env!("ANDROID_HOME");
  format!("{andy}/build-tools/34.0.0/aapt")
}

pub fn run_aapt(apk: String) -> String {
  let output = String::from_utf8(
    Command::new(get_aapt_path())
      .args(["d", "badging"])
      .arg(apk)
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output()
      .unwrap()
      .stdout,
  )
  .unwrap()
  .replace("\r", "");

  let data = output.split("\n")
    .filter(|x| x.starts_with("package:"))
    .collect::<Vec<_>>()[0];

  let data = data.replace("package: ", "");
  let data = data.split(" ").collect::<Vec<_>>()[0];

  let data = data.split("=").collect::<Vec<_>>()[1];

  let data = data[1..data.len()-1].to_string();
  
  return data;
}
