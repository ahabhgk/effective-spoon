use std::future::Future;

pub trait Plugin<Cx, Request> {
  type Response;
  type Error;

  type Future<'cx>: Future<Output = Result<Self::Response, Self::Error>>
    + Send
    + 'cx
  where
    Cx: 'cx,
    Self: 'cx;

  fn call<'cx, 's>(
    &'s mut self,
    cx: &'cx mut Cx,
    req: Request,
  ) -> Self::Future<'cx>
  where
    's: 'cx;
}
