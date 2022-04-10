use aczor_shared::{
  anyhow::Error,
  async_trait,
  options::Options,
  plugin::{Plugin, ResolveArgs, ResolveResult, Resolver},
};
use url::Url;

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
    _options: &Options,
    args: ResolveArgs<'_, '_>,
  ) -> Result<Option<ResolveResult>, Error> {
    let importer = args.importer;
    let dependency = args.dependency;
    let base_url = Url::from_file_path(&importer.path).ok();
    let url = Url::options()
      .base_url(base_url.as_ref())
      .parse(&dependency.specifier)?;
    let path = url.to_file_path().unwrap();
    Ok(Some(ResolveResult { path }))
  }
}
