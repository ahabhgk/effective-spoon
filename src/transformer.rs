use crate::asset::Asset;

pub trait Transformer {
  fn transform(args: TransformArgs) -> TransformResult;
}

pub struct TransformArgs {
  pub asset: Asset,
}

pub struct TransformResult {
  pub asset: Asset,
}
