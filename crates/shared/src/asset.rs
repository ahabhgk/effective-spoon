use std::any::Any;

use url::Url;

#[derive(Debug)]
pub struct Asset {
  pub id: Url,
  content: Vec<u8>,
  data: Option<Box<dyn Any + Send + Sync>>,
}

impl Asset {
  pub fn new(id: Url) -> Self {
    Self {
      id,
      content: Vec::new(),
      data: None,
    }
  }

  pub fn set_content(&mut self, content: Vec<u8>) {
    self.content = content;
  }
}
