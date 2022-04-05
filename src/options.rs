use std::path::PathBuf;

pub struct Options {
  pub entries: Vec<String>,
  pub cwd: PathBuf,
}
