use std::path::PathBuf;

use anyhow::Error;
use async_trait::async_trait;

use crate::{asset::Asset, dependency::Dependency};

pub trait Plugin: Send + Sync {
  fn name(&self) -> &'static str;
}

#[async_trait]
pub trait Resolver: Plugin {
  async fn resolve(&self, args: ResolveArgs) -> Result<Option<ResolveResult>, Error>;
}

pub struct ResolveArgs<'a, 's, 'd> {
  pub importer: &'a Asset,
  pub specifier: &'s str,
  pub dependency: &'d Dependency,
}

pub struct ResolveResult {
  pub path: PathBuf,
}

#[async_trait]
pub trait Transformer: Plugin {
  async fn transform(&self, args: TransformArgs) -> Result<Option<TransformResult>, Error>;
}

pub struct TransformArgs {
  pub asset: Asset,
}

pub struct TransformResult {
  pub asset: Asset,
}
