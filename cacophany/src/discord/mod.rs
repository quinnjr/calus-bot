//
//

//!

use derive_builder::Builder;
use threadpool::ThreadPool;

use std::{
  collections::HashMap,
  sync::{ Arc, Mutex, RwLock },
  marker::PhantomData
};

use crate::{
  http::{ Client as HttpClient, ClientBuilder as HttpClientBuilder },
  models,
  prelude::internal::*
};


mod error;
mod event_handler;

pub use self::{
  error::ClientError,
  event_handler::EventHandler
};

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Client<'re> {
  data: Arc<HashMap<CowStr<'re>, Value>>,
  shards: Arc<HashMap<CowStr<'re>, &'re [u8; 2]>>,
  threadpool: ThreadPool,
}

impl<'re> Default for Client<'re> {
  fn default() -> Self {
    Self {
      data: Arc::new(HashMap::new()),
      shards: Arc::new(HashMap::new()),
      threadpool: ThreadPool::new(5)
    }
  }
}

impl<'re> Client<'re> {
  pub fn new(token: CowStr<'re>, handler: EventHandler) -> Result<Self>
    where EventHander: Send + Sync + 'static
  {
    let token = token.trim();

    let token = if !token.starts_with("Bot ") {
      &["Bot ", token].join("")
    } else {
      token
    };

    let http = HttpClientBuilder::default()
      .token(token.into())
      .build()?;

    Ok(Self::default())
  }
}
