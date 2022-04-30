use std::any::Any;

use smol_str::SmolStr;

#[derive(Debug)]
pub struct Dependency {
  pub is_async: bool,
  pub specifier: SmolStr,
  pub data: Option<Box<dyn Any + Send + Sync>>,
}
