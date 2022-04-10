use std::path::PathBuf;

use anyhow::Error;

use crate::{
  asset::Asset,
  dependency::Dependency,
  plugin::{InputFS, LoadArgs, Loader, ResolveArgs, Resolver, Transformer, Transforming},
};

pub struct Options {
  pub entries: Vec<String>,
  pub root: PathBuf,
  pub input_fs: Box<dyn InputFS>,
  pub resolvers: Vec<Box<dyn Resolver>>,
  pub loaders: Vec<Box<dyn Loader>>,
  pub transformers: Vec<Box<dyn Transformer>>,
}

impl Options {
  pub async fn run_resolvers(
    &self,
    importer: &Asset,
    dependency: &Dependency,
  ) -> Result<PathBuf, Error> {
    for resolver in &self.resolvers {
      if let Some(result) = resolver
        .resolve(
          self,
          ResolveArgs {
            importer,
            dependency,
          },
        )
        .await?
      {
        return Ok(result.path);
      }
    }

    panic!("unresolvable: {:?} {:?}", importer, dependency);
  }

  pub async fn run_loaders(&self, path: PathBuf) -> Result<Asset, Error> {
    for loader in &self.loaders {
      if let Some(result) = loader.load(self, LoadArgs { path: &path }).await? {
        return Ok(Asset {
          path,
          content: result.content,
        });
      }
    }

    panic!("unloadable: {:?}", path);
  }

  pub async fn run_transformers(&self, asset: Asset) -> Result<Transforming, Error> {
    let mut transforming = Transforming {
      asset,
      dependencies: Vec::new(),
    };

    for transformer in &self.transformers {
      transformer.transform(self, &mut transforming).await?;
    }

    Ok(transforming)
  }
}
