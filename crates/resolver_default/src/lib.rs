use aczor_shared::{
  anyhow::Error,
  async_trait,
  options::Options,
  plugin::{Plugin, ResolveArgs, ResolveResult, Resolver},
};

pub struct ResolverDefault;

impl Plugin for ResolverDefault {
  fn name(&self) -> &'static str {
    "resolver_default"
  }
}

#[async_trait]
impl Resolver for ResolverDefault {
  async fn resolve(
    &self,
    options: &Options,
    args: ResolveArgs<'_, '_>,
  ) -> Result<Option<ResolveResult>, Error> {
    Ok(None)
  }
}
