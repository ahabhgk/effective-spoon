use crate::module::ModuleBuilder;

pub trait Transformer {
  fn transform(args: TransformArgs) -> TransformResult;
}

pub struct TransformArgs {
  pub module: ModuleBuilder,
}

pub struct TransformResult {
  pub modules: Vec<ModuleBuilder>,
}
