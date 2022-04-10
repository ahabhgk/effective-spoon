use std::{collections::HashMap, path::PathBuf};

use anyhow::Error;
use async_recursion::async_recursion;
use futures::future::try_join_all;
use petgraph::{graph::NodeIndex, Graph};

use crate::{asset::Asset, dependency::Dependency, options::Options, plugin::Transforming};

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
    let entries = self
      .options
      .entries
      .clone()
      .into_iter()
      .map(|entry| Dependency {
        is_async: false,
        specifier: entry,
      })
      .collect();
    self.build_children(self.root_idx, entries).await?;
    Ok(())
  }

  #[async_recursion]
  async fn build_children(
    &mut self,
    asset_idx: NodeIndex,
    dependencies: Vec<Dependency>,
  ) -> Result<(), Error> {
    let importer = &self.graph[asset_idx];

    let futs = dependencies.into_iter().map(|dependency| async {
      let path = self.options.run_resolvers(importer, &dependency).await?;

      if self.idx_by_path.contains_key(&path) {
        return Ok(None);
      }

      let asset = self.options.run_loaders(path).await?;
      let transforming = self.options.run_transformers(asset).await?;
      Ok::<Option<(Transforming, Dependency)>, Error>(Some((transforming, dependency)))
    });
    let children: Vec<Option<(Transforming, Dependency)>> = try_join_all(futs).await?;

    let children: Vec<(NodeIndex, Vec<Dependency>)> = children
      .into_iter()
      .filter_map(|child| child)
      .map(|(transforming, dependency)| {
        (
          self.add_asset(asset_idx, transforming.asset, dependency),
          transforming.dependencies,
        )
      })
      .collect();
    for child in children {
      self.build_children(child.0, child.1).await?;
    }

    Ok(())
  }

  fn add_asset(&mut self, from_idx: NodeIndex, to_node: Asset, edge: Dependency) -> NodeIndex {
    let path = to_node.path.clone();
    let idx = self.graph.add_node(to_node);
    self.idx_by_path.insert(path, idx);
    self.graph.add_edge(from_idx, idx, edge);
    idx
  }
}
