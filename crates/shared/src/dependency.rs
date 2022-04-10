#[derive(Debug)]
pub struct Dependency {
  pub is_async: bool,
  pub specifier: String,
}
