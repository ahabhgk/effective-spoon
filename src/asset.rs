use std::path::PathBuf;

use crate::dependency::Dependency;

pub struct Asset {
  pub path: PathBuf,
  pub content: Vec<u8>,
  pub dependency_paths: Vec<String>,
}
