// use aczor_shared::{
//   anyhow::Error,
//   async_trait,
//   plugin::{Plugin, ResolveArgs, ResolveResult, Resolver},
//   runner::Options,
// };
use url::Url;

// pub struct ResolverDefault;

// impl Plugin for ResolverDefault {
//   fn name(&self) -> &'static str {
//     "aczor_resolver_default"
//   }
// }

// #[async_trait]
// impl Resolver for ResolverDefault {
//   async fn resolve(
//     &self,
//     args: ResolveArgs<'_, '_>,
//   ) -> Result<Option<ResolveResult>, Error> {
//     let importer = args.importer;
//     let dependency = args.dependency;
//     let base_url = Url::from_file_path(&importer.path).ok();
//     let url = Url::options()
//       .base_url(base_url.as_ref())
//       .parse(&dependency.specifier)?;
//     let path = url.to_file_path().unwrap();
//     Ok(Some(ResolveResult { path }))
//   }
// }

#[test]
fn tt() {
  let url = Url::parse("file:///home/").unwrap();
  dbg!(&url);
  // let r = Url::parse("file://./index.js").unwrap();
  // dbg!(r);
  let id = Url::options()
    .base_url(Some(&url))
    .parse("file:./index.js")
    .unwrap();
  // let id = id.to_file_path().unwrap();
  dbg!(id);
}
