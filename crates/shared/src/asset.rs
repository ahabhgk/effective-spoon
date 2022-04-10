use std::path::PathBuf;

#[derive(Debug)]
pub struct Asset {
  pub path: PathBuf,
  pub content: Vec<u8>,
}
