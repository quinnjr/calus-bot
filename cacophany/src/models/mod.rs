//
//

//!

pub mod account;
pub mod audit_log;
pub mod gateway;
pub mod guild;
pub mod user;

use self::{
  gateway::Activity,
  prelude::*,
  user::Status
};

pub type Presence<'re> = (Option<Activity<'re>>, Status);
pub type Snowflake = u32;

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
pub struct Payload<'re> {
  op: u8,
  d: Value,
  s: Option<u32>,
  t: Option<CowStr<'re>>
}

impl<'re> Default for Payload<'re> {
  fn default() -> Self {
    Self {
      op: 0,
      d: Value::default(),
      s: None,
      t: None
    }
  }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Webhook;

mod prelude {
  //!
  pub(crate) use chrono::Duration;
  pub(crate) use derive_builder::Builder;
  pub(crate) use failure::{ Fail, ResultExt };
  pub(crate) use serde::{
    de::{ self },
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    ser::SerializeStruct
  };
  pub(crate) use serde_json::Value;
  pub(crate) use url::Url;

  pub(crate) use std::{
    borrow::Cow,
    clone::Clone,
    cmp::{ PartialEq, PartialOrd, Eq, Ord },
    collections::HashMap,
    convert::{ Into, TryInto, TryFrom },
    default::Default,
    fmt::{ self, Debug, Display, Formatter },
    marker::{ Copy, PhantomData },
    option::Option,
    result::Result as StdResult,
    sync::Arc
  };

  #[doc(no_inline)]
  pub(crate) use crate::{
    error::Error,
    http::Client as HttpClient,
    models::Snowflake,
    result::Result,
    util::CowStr
  };
}
