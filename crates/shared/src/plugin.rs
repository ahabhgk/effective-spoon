use std::fmt::Display;

use async_trait::async_trait;
use url::Url;

use crate::{asset::Asset, dependency::Dependency, error::Result};

pub trait Plugin: Send + Sync + Display {}

#[async_trait]
pub trait Resolver: Plugin {
  fn apply(&self, importer_id: &Url, specifier: &str) -> bool;

  async fn resolve(&self, args: ResolveArgs<'_, '_>) -> Result<ResolveResult>;
}

pub struct ResolveArgs<'a, 'd> {
  pub importer: &'a Asset,
  pub dependency: &'d Dependency,
}

pub struct ResolveResult {
  pub asset_id: Url,
}

#[async_trait]
pub trait Loader: Plugin {
  fn apply(&self, id: &Url) -> bool;

  async fn load(&self, args: LoadArgs<'_>) -> Result<LoadResult>;
}

pub struct LoadArgs<'i> {
  pub asset_id: &'i Url,
}

pub struct LoadResult {
  pub content: Vec<u8>,
}

#[async_trait]
pub trait Transformer: Plugin {
  fn apply(&self, id: &Url) -> bool;

  async fn transform(&self, args: &mut Transforming) -> Result<()>;
}

pub struct Transforming {
  pub asset: Asset,
  pub dependencies: Vec<Dependency>,
}
