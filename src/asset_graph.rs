use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use anyhow::Error;
use async_recursion::async_recursion;
use futures::future::try_join_all;
use petgraph::{graph::NodeIndex, Graph};

use crate::{asset::Asset, dependency::Dependency, options::Options};

pub struct AssetGraph<'o> {
  graph: Graph<Asset, Dependency>,
  root_idx: NodeIndex,
  idx_by_path: HashMap<PathBuf, NodeIndex>,
  options: &'o Options,
}

impl<'o> AssetGraph<'o> {
  pub fn new(options: &'o Options) -> Self {
    let root = Asset {
      path: options.root.clone(),
      content: b"__ROOT__".to_vec(),
      dependency_specifiers: options.entries.clone(),
    };
    let mut graph = Graph::new();
    let root_idx = graph.add_node(root);

    Self {
      graph,
      root_idx,
      idx_by_path: HashMap::new(),
      options,
    }
  }

  pub async fn build(&mut self) -> Result<(), Error> {
    self.build_children(self.root_idx).await?;
    Ok(())
  }

  #[async_recursion]
  async fn build_children(&mut self, asset_idx: NodeIndex) -> Result<(), Error> {
    let asset = &self.graph[asset_idx];
    let specifiers = &asset.dependency_specifiers;

    let futs = specifiers.iter().map(|specifier| async {
      let path = self.options.run_resolvers(importer, specifier).await?;

      if self.idx_by_path.contains_key(&path) {
        return Ok(None);
      }

      let asset = Asset::load(path).await?;
      let asset = self.transform(asset).await?;
      Ok::<Option<Asset>, Error>(Some(asset))
    });
    let assets: Vec<Option<Asset>> = try_join_all(futs).await?;

    let idxs: Vec<NodeIndex> = assets
      .into_iter()
      .filter_map(|asset| asset)
      .map(|asset| self.add_asset(asset_idx, asset))
      .collect();
    for idx in idxs {
      self.build_children(idx).await?;
    }

    Ok(())
  }

  fn add_asset(&mut self, from_idx: NodeIndex, to_node: Asset) -> NodeIndex {
    let path = to_node.path.clone();
    let idx = self.graph.add_node(to_node);
    self.idx_by_path.insert(path, idx);
    self
      .graph
      .add_edge(from_idx, idx, Dependency { is_async: false });
    idx
  }

  async fn run_resolve(&self, importer: &Path, specifier: &str) -> Result<PathBuf, Error> {
    todo!()
  }

  async fn transform(&self, asset: Asset) -> Result<Asset, Error> {
    Ok(asset)
  }
}
