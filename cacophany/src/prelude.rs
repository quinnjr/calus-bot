//
//

//!
//!

pub use crate::{
  cache::Cache,
  client::Client as CacophanyClient,
  consts::{
    DISCORD_API_ENTRYPOINT,
    DISCORD_API_VERSION,
    DISCORD_GATEWAY_BOT_ENTRYPOINT,
    DISCORD_GATEWAY_ENTRYPOINT,
    DISCORD_GATEWAY_VERSION,
    EMBED_MAX_LENGTH,
    JOIN_MESSAGES,
    LARGE_THRESHOLD,
    MESSAGE_CODE_LIMIT,
    USER_AGENT
  },
  error::Error as CacophanyError,
  gateway,
  http::Client as CacophanyHttpClient,
  models,
  permission::Permission,
  result::Result as CacophanyResult,
  util::{ MessageExt as Message, ValueExt as Value }
};

pub(crate) mod internal {
  //! Internal prelude module for lazyness.
  #![allow(unused_imports)]
  #![doc(no_inline)]

  pub(crate) use chrono::{
    DateTime,
    Duration,
    Utc
  };
  pub(crate) use derive_builder::Builder;
  pub(crate) use failure::Fail;
  pub(crate) use http::{
    HeaderMap,
    HeaderValue,
    Method,
    Request,
    Response,
    Uri
  };
  pub(crate) use log::*;
  pub(crate) use serde::{ Deserialize, Deserializer, Serialize, Serializer };

  pub(crate) use std::{
    borrow::Cow,
    clone::Clone,
    cmp::{ Eq, Ord, PartialEq, PartialOrd },
    convert::{ From, TryFrom, Into, TryInto },
    default::Default,
    fmt::{ self, Debug, Display },
    option::Option,
    result::Result as StdResult,
    time::Duration as StdDuration
  };

  pub(crate) use crate::{
    consts::{
      DISCORD_API_ENTRYPOINT,
      DISCORD_API_VERSION,
      DISCORD_GATEWAY_BOT_ENTRYPOINT,
      DISCORD_GATEWAY_ENTRYPOINT,
      DISCORD_GATEWAY_VERSION,
      EMBED_MAX_LENGTH,
      JOIN_MESSAGES,
      LARGE_THRESHOLD,
      MESSAGE_CODE_LIMIT,
      USER_AGENT
    },
    error::Error,
    result::Result,
    permission,
    util::{ CowStr, MessageExt as Message, ValueExt as Value }
  };
}
