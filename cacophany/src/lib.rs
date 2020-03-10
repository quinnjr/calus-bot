//
//

//! ![Logo](logo.jpg)
//! ---
//!
//! ![Docs version](https://docs.rs/cacophany/badge.svg)
//!
//! ---
//! The use of the lifetime `'re` throughout this library is based upon
//! the context that it is invoked in, typically either a re**quest**
//! or a re**sponse**.

#![allow(dead_code)]

#![cfg_attr(debug_assertions, allow(unused_imports))]
#![cfg_attr(debug_assertions, feature(trace_macros))]
#![cfg_attr(not(debug_assertions), deny(unused_imports))]

mod cache;
mod consts;
mod permission;
mod util;

pub use self::{
  cache::Cache,
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
  permission::Permission,
  util::{ MessageExt as Message, ValueExt as Value }
};

pub mod discord;
pub mod error;
pub mod gateway;
pub mod http;
pub mod models;
pub mod prelude;

pub use discord::Client;

pub mod result {
  use crate::error::Error;
  pub type Result<T> = ::std::result::Result<T, Error>;
}

#[allow(rust_2018_idioms)]
extern crate self as cacophony;
