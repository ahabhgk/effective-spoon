use std::path::PathBuf;

use anyhow::Error;

use crate::{
  asset::Asset,
  dependency::Dependency,
  plugin::{ResolveArgs, Resolver, Transformer},
};

pub struct Options {
  pub entries: Vec<String>,
  pub root: PathBuf,
  pub resolvers: Vec<Box<dyn Resolver>>,
  pub transformers: Vec<Box<dyn Transformer>>,
}

impl Options {
  pub async fn run_resolvers(
    &self,
    importer: &Asset,
    specifier: &str,
    dependency: &Dependency,
  ) -> Result<PathBuf, Error> {
    for resolver in &self.resolvers {
      if let Some(result) = resolver
        .resolve(ResolveArgs {
          importer,
          specifier,
          dependency,
        })
        .await?
      {
        return Ok(result.path);
      }
    }

    panic!("unresolved specifier: {}", specifier);
  }
}

// pub trait InputFS {
//   fn r
// }
