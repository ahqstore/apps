use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Org {
  login: String,
}

pub fn has_org_members(org: &str, user: &str) -> bool {
  let calc: Option<bool> = (|| {
    let client = Client::new();

    let authors = client
      .get(format!("https://api.github.com/orgs/{org}/public_members"))
      .header("User-Agent", "reqwest/AHQ Store")
      .header("Authorization", format!("Bearer {}", env!("GH_TOKEN")))
      .send()
      .ok()?
      .json::<Vec<Org>>()
      .ok()?;

    Some(
      authors
        .iter()
        .find(|author| &author.login == user)
        .is_some(),
    )
  })();

  calc.unwrap_or(false)
}
