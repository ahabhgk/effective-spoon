use std::path::{Path, PathBuf};

use anyhow::Error;
use async_trait::async_trait;

use crate::{asset::Asset, dependency::Dependency, options::Options};

pub trait Plugin: Send + Sync {
  fn name(&self) -> &'static str;
}

#[async_trait]
pub trait InputFS: Plugin {
  async fn read(&self, path: &Path) -> Result<Vec<u8>, Error>;
}

#[async_trait]
pub trait Resolver: Plugin {
  async fn resolve(
    &self,
    options: &Options,
    args: ResolveArgs<'_, '_>,
  ) -> Result<Option<ResolveResult>, Error>;
}

pub struct ResolveArgs<'a, 'd> {
  pub importer: &'a Asset,
  pub dependency: &'d Dependency,
}

pub struct ResolveResult {
  pub path: PathBuf,
}

#[async_trait]
pub trait Loader: Plugin {
  async fn load(&self, options: &Options, args: LoadArgs<'_>) -> Result<Option<LoadResult>, Error>;
}

pub struct LoadArgs<'a> {
  pub path: &'a Path,
}

pub struct LoadResult {
  pub content: Vec<u8>,
}

#[async_trait]
pub trait Transformer: Plugin {
  async fn transform(
    &self,
    options: &Options,
    transforming: &mut Transforming,
  ) -> Result<(), Error>;
}

pub struct Transforming {
  pub asset: Asset,
  pub dependencies: Vec<Dependency>,
}
