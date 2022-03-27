use std::path::PathBuf;

use crate::dependency::Dependency;

pub trait Resolver {
  fn resolve(&mut self, args: ResolveArgs) -> ResolveResult;
}

pub struct ResolveArgs {
  pub specifier: PathBuf,
  pub dependency: Dependency,
}

pub struct ResolveResult {
  pub path: PathBuf,
}
