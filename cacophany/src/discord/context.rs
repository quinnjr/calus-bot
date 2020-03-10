//
//

//!

use crate::internal::prelude::*;

use std::sync::RwLock;

#[derive(Copy, Clone, Debug, Builder)]
pub struct Context<'re, T> {
  cache: impl crate::cache::Cache,
  data: Arc<RwLock<HashMap<CowStr<'re>, T>>>
  shard:
  shard_id: u32,
  http: &'re crate::http::Client<'re>
}

impl<'re> Context<'re, T> {
  pub fn new<C>(
    data: Arc<RwLock<HashMap<CowStr<'re>, T>>>,
    shard: &C,
    shard_id: u32,
    http: &'re crate::http::Client<'re>
  )-> Self
    where C: impl crate::cache::Cache + Send + Sync + 'static
  {
    Self {

    }
  }
}
