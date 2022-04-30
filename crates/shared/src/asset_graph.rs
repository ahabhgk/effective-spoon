use std::collections::HashMap;

use async_recursion::async_recursion;
use futures::future::try_join_all;
use petgraph::{graph::NodeIndex, Graph};
use smol_str::SmolStr;
use url::Url;

use crate::{
  asset::Asset,
  dependency::Dependency,
  driver::{Driver, FS},
  error::{Error, Result},
  plugin::Transforming,
};

pub struct AssetGraph {
  graph: Graph<Asset, Dependency>,
  root_ix: NodeIndex,
  ix_by_id: HashMap<Url, NodeIndex>,
}

impl AssetGraph {
  pub fn new(root_id: impl AsRef<str>) -> Self {
    let root_id = Url::parse(root_id.as_ref()).unwrap();
    let root = Asset::new(root_id.clone());
    let mut graph = Graph::new();
    let root_ix = graph.add_node(root);
    let mut ix_by_id = HashMap::new();

    ix_by_id.insert(root_id, root_ix);

    Self {
      graph,
      root_ix,
      ix_by_id,
    }
  }

  pub async fn build<F: FS>(
    &mut self,
    entries: Vec<impl Into<SmolStr>>,
    driver: &Driver<'_, '_, '_, F>,
  ) -> Result<()> {
    let args = entries
      .into_iter()
      .map(|entry| Dependency {
        is_async: false,
        specifier: entry.into(),
        data: None,
      })
      .collect();
    self.build_children(self.root_ix, args, driver).await?;
    Ok(())
  }

  #[async_recursion]
  async fn build_children<F: FS>(
    &mut self,
    asset_ix: NodeIndex,
    deps: Vec<Dependency>,
    driver: &Driver<F>,
  ) -> Result<()> {
    let importer = &self.graph[asset_ix];

    let futs = deps.into_iter().map(|dep| async {
      let resolved = driver.resolve(importer, &dep).await?;

      if self.ix_by_id.contains_key(&resolved.asset_id) {
        return Ok(None);
      }

      let loaded = driver.load(&resolved.asset_id).await?;
      let mut asset = Asset::new(resolved.asset_id);
      asset.set_content(loaded.content);
      let transforming = driver.transform(asset).await?;
      Ok::<Option<(Transforming, Dependency)>, Error>(Some((transforming, dep)))
    });
    let children: Vec<Option<(Transforming, Dependency)>> =
      try_join_all(futs).await?;

    let children: Vec<(NodeIndex, Vec<Dependency>)> = children
      .into_iter()
      .filter_map(|child| child)
      .map(|(transforming, dependency)| {
        (
          self.add_asset(asset_ix, transforming.asset, dependency),
          transforming.dependencies,
        )
      })
      .collect();
    for child in children {
      self.build_children(child.0, child.1, driver).await?;
    }

    Ok(())
  }

  fn add_asset(
    &mut self,
    from_ix: NodeIndex,
    to_node: Asset,
    edge: Dependency,
  ) -> NodeIndex {
    let id = to_node.id.clone();
    let ix = self.graph.add_node(to_node);
    self.ix_by_id.insert(id, ix);
    self.graph.add_edge(from_ix, ix, edge);
    ix
  }
}
