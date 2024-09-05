#[cfg(feature = "load_bytes")]
mod load;
mod parser;
#[cfg(feature = "remove_manifest")]
mod remove;
mod types;

pub mod shared;

use parser::*;
use types::*;

fn main() {
  #[cfg(feature = "load_bytes")]
  load::run();

  #[cfg(feature = "remove_manifest")]
  remove::remove_manifest();

  parser();
}