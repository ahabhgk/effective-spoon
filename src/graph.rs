use crate::{dependency::Dependency, module::Module};

pub struct Graph {
  inner: petgraph::Graph<Module, Dependency>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      inner: petgraph::Graph::new(),
    }
  }

  pub fn build(&mut self) {}
}
