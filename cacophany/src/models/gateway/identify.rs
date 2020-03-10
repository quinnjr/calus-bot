//
//

//!

use crate::models::prelude::*;

use super::UpdateStatus;

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Identify<'re> {
  token: CowStr<'re>,
  properties: ConnectionProperties<'re>,
  compress: Option<bool>,
  large_threshold: Option<u32>,
  shard: Option<&'re [u32; 2]>,
  #[serde(skip_serializing_if = "Option::is_none")]
  presence: Option<UpdateStatus<'re>>,
  guild_subscriptions: Option<bool>
}

impl<'re> Default for Identify<'re> {
  fn default() -> Self {
    Self {
      token: Cow::Borrowed(""),
      properties: ConnectionProperties::default(),
      compress: Some(true),
      large_threshold: Some(50),
      shard: None,
      presence: None,
      guild_subscriptions: Some(true)
    }
  }
}

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
pub struct ConnectionProperties<'re> {
  #[serde(rename = "$os")]
  os: CowStr<'re>,
  #[serde(rename = "$browser")]
  browser: CowStr<'re>,
  #[serde(rename = "$device")]
  device: CowStr<'re>
}

impl<'re> Default for ConnectionProperties<'re> {
  fn default() -> Self {
    Self {
      os: Cow::Borrowed(""),
      browser: Cow::Borrowed(""),
      device: Cow::Borrowed("")
    }
  }
}
