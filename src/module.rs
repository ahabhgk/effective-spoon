use std::path::PathBuf;

pub struct Module {
  path: PathBuf,
  code: String,
}

pub struct ModuleBuilder {
  path: PathBuf,
  code: Option<String>,
}
