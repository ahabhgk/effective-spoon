use std::path::{Path, PathBuf};

use anyhow::Error;
use petgraph::graph::NodeIndex;
use tokio::fs;

use crate::{asset_graph::AssetGraph, dependency::Dependency};

pub struct Asset {
  pub path: PathBuf,
  pub content: Vec<u8>,
  pub dependency_specifiers: Vec<String>,
}

impl Asset {
  pub async fn load(path: PathBuf) -> Result<Self, Error> {
    let content = fs::read(&path).await?;
    Ok(Self {
      path,
      content,
      dependency_specifiers: Vec::new(),
    })
  }
}
