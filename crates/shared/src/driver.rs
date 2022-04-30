use std::path::Path;

use async_recursion::async_recursion;
use async_trait::async_trait;
use url::Url;

use crate::{
  asset::Asset,
  dependency::Dependency,
  error::Result,
  plugin::{
    LoadArgs, LoadResult, Loader, ResolveArgs, ResolveResult, Resolver,
    Transformer, Transforming,
  },
};

#[async_trait]
pub trait FS: Send + Sync {
  async fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>>;

  async fn write(
    &self,
    path: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
  ) -> Result<()>;
}

pub struct Driver<'r, 'l, 't, F: FS> {
  pub fs: F,
  pub resolvers: Vec<&'r dyn Resolver>,
  pub loaders: Vec<&'l dyn Loader>,
  pub transformers: Vec<&'t dyn Transformer>,
}

impl<F: FS> Driver<'_, '_, '_, F> {
  pub async fn resolve(
    &self,
    importer: &Asset,
    dependency: &Dependency,
  ) -> Result<ResolveResult> {
    for resolver in self.resolvers.iter() {
      if resolver.apply(&importer.id, &dependency.specifier) {
        return resolver
          .resolve(ResolveArgs {
            importer,
            dependency,
          })
          .await;
      }
    }

    let id = Url::options()
      .base_url(Some(&importer.id))
      .parse(&dependency.specifier)?;
    Ok(ResolveResult { asset_id: id })
  }

  pub async fn load(&self, id: &Url) -> Result<LoadResult> {
    for loader in self.loaders.iter() {
      if loader.apply(id) {
        return loader.load(LoadArgs { asset_id: id }).await;
      }
    }

    let path = id.to_file_path().unwrap();
    let content = self.fs.read(path).await?;
    Ok(LoadResult { content })
  }

  pub async fn transform(&self, asset: Asset) -> Result<Transforming> {
    let transforming = Transforming {
      asset,
      dependencies: Vec::new(),
    };

    #[async_recursion]
    async fn run_transformers(
      mut transforming: Transforming,
      pre_ixs: Vec<usize>,
      transformers: &Vec<&dyn Transformer>,
    ) -> Result<Transforming> {
      let (cur_ixs, cur_transformers): (Vec<usize>, Vec<&dyn Transformer>) =
        transformers
          .iter()
          .enumerate()
          .filter(|(ix, transformer)| {
            !pre_ixs.contains(ix) && transformer.apply(&transforming.asset.id)
          })
          .unzip();
      for transformer in cur_transformers.iter() {
        if transformer.apply(&transforming.asset.id) {
          transformer.transform(&mut transforming).await?;
        }
      }

      run_transformers(transforming, cur_ixs, transformers).await
    }

    run_transformers(transforming, Vec::new(), &self.transformers).await
  }
}
