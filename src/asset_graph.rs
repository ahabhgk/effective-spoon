use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use anyhow::Error;
use petgraph::{graph::NodeIndex, Graph};
use tokio::fs;

use crate::{asset::Asset, dependency::Dependency, options::Options};

pub struct AssetGraph {
  graph: Graph<Asset, Dependency>,
  root_idx: NodeIndex,
  idx_by_path: HashMap<PathBuf, NodeIndex>,
}

impl AssetGraph {
  pub fn new(options: &Options) -> Self {
    let cwd = options.cwd.clone();
    let root = Asset {
      path: cwd,
      content: String::from("@@ROOT").into_bytes(),
      dependency_paths: options.entries.clone(),
    };
    let mut graph = Graph::new();
    let root_idx = graph.add_node(root);

    Self {
      graph,
      root_idx,
      idx_by_path: HashMap::new(),
    }
  }

  pub async fn build(&mut self) -> Result<(), Error> {
    self.build_children(self.root_idx).await?;
    Ok(())
  }

  async fn build_asset(&mut self, path: &str) -> Result<NodeIndex, Error> {
    let path = self.resolve(path).await?;

    if let Some(&idx) = self.idx_by_path.get(&path) {
      return Ok(idx);
    }

    let content = fs::read(&path).await?;
    let asset = Asset {
      path,
      content,
      dependency_paths: Vec::new(),
    };

    let asset = self.transform(asset).await?;

    let node_idx = self.add_asset(asset);
    Ok(node_idx)
  }

  async fn build_children(&mut self, asset_idx: NodeIndex) -> Result<Vec<NodeIndex>, Error> {
    let deps = &self.graph[asset_idx].dependency_paths.clone();

    let mut idxs = Vec::new();
    for dep in deps {
      let dep_idx = self.build_asset(dep).await?;
      self
        .graph
        .add_edge(asset_idx, dep_idx, Dependency { is_async: false });
      idxs.push(dep_idx);
    }

    Ok(idxs)
  }

  fn add_asset(&mut self, node: Asset) -> NodeIndex {
    let path = node.path.clone();
    let idx = self.graph.add_node(node);
    self.idx_by_path.insert(path, idx);
    idx
  }

  fn get_cwd(&self) -> &Path {
    &self.graph[self.root_idx].path
  }

  async fn resolve(&self, path: &str) -> Result<PathBuf, Error> {
    let cwd = self.get_cwd();
    Ok(cwd.join(path))
  }

  async fn transform(&self, asset: Asset) -> Result<Asset, Error> {
    Ok(asset)
  }
}
