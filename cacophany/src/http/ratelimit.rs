//
//

//!
//!

use chrono::{
  DateTime,
  Duration,
  Utc
};
use http::{ HeaderMap, Request, Response, StatusCode };
use lazy_static::lazy_static;
use serde::{ Deserialize, Serialize };

use std::{
  borrow::Cow,
  convert::TryInto,
  thread,
  time::Duration as StdDuration
};

use crate::result::Result;

#[inline]
fn offset(headers: &HeaderMap) -> Option<Duration> {
  let now = Utc::now().timestamp();

  if let Some(servertime_header) = headers.get("Date") {
    let s = servertime_header.to_str()
      .expect("Failed to parse server header Date field")
      .replace("GMT", "+0000");

    if let Ok(servertime) = DateTime::parse_from_str(&s,
      "%a, %d %b %Y $T %z")
    {
      let offset = servertime.timestamp();
      let diff = offset - now;

      Some(diff);
    };
    None
  } else {
    None
  }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RateLimit<'r> {
  global: isize,
  limit: isize,
  remaining: isize,
  reset: chrono::DateTime<chrono::Utc>,
  #[serde(with = "crate::util::duration")]
  reset_after: chrono::Duration,
  bucket: Cow<'r, str>
}

impl<'r> RateLimit<'r> {
  pub fn pre_hook(&mut self, req: &Request<()>) {
    if self.limit != 0 {
      let offset: isize = offset(req.headers())
        .unwrap_or(Duration::zero())
        .num_seconds() as isize;
      let now = Utc::now().timestamp() as isize;
      let curr: isize = (now - offset).try_into().unwrap();

      if curr.gt(&(self.reset.timestamp() as isize)) {
        self.remaining = self.limit;
        return;
      }

      let diff = (self.remaining - curr) as u64;

      if self.remaining == 0 {
        let delay = diff * 1000;

        thread::sleep(StdDuration::from_millis(delay));
      }

      self.remaining = self.remaining - 1;
    }
  }

  pub fn post_hook(&mut self, res: &'r Response<()>,
    req: &Request<()>) -> Result<()>
  {
    let headers = res.headers();

    if let Some(limit) = headers.get("x-ratelimit-limit") {
      self.limit = limit.to_str()?.parse()?;
    }

    if let Some(remaining) = headers.get("x-ratelimit-remaining") {
      self.remaining = remaining.to_str()?.parse()?;
    }

    if let Some(reset) = headers.get("x-ratelimit-reset") {
      self.reset = reset.to_str()?.parse()?;
    }

    if let Some(reset_after) = headers.get("x-ratelimit-reset-after") {
      self.reset_after = Duration::seconds(reset_after.to_str()?.parse()?);
    }

    if let Some(bucket) = headers.get("x-ratelimit-bucket") {
      self.bucket = bucket.to_str()?.into();
    }

    Ok(if res.status() == StatusCode::TOO_MANY_REQUESTS {
      if let Some(retry) = headers.get("retry-after") {
        thread::sleep(StdDuration::from_secs(retry.to_str()?.parse()?));
      };
    })
  }
}
