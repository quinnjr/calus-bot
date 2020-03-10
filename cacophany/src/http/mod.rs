//

//!
//!

use tokio::net::TcpStream;

use crate::prelude::internal::*;

pub mod ratelimit;

#[derive(Clone, Debug, Builder, Deserialize, Serialize)]
pub struct Client<'re> {
  token: CowStr<'re>,
  gateway_url: CowStr<'re>,
  ws_gateway_url: CowStr<'re>,
  inner: Option<()>
}

impl<'re> Client<'re> {
  ///
  ///
  pub fn request<T: Sized + Sync, R>(&mut self, method: Method,
    path: CowStr<'re>, headers: Option<HeaderMap>,
    body: Option<T>) -> Result<Response<R>>
  {
    unimplemented!()
  }

  ///
  ///
  pub fn get<R>(&mut self, path: CowStr<'re>, headers: Option<HeaderMap>)
    -> Result<Response<R>>
  {
    unimplemented!()
  }

  ///
  ///
  pub fn post<R>(&mut self, path: CowStr<'re>, headers: Option<HeaderMap>)
    -> Result<Response<R>>
  {
    unimplemented!()
  }
}
