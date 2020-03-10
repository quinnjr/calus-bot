//
//

use derive_builder::Builder;
use serde::{ Deserialize, Serialize };
use serde_json::Value;

use std::borrow::Cow;

use crate::{
  models::Snowflake,
  util::CowStr
};

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Members<'re> {
  guild_id: Snowflake,
  query: CowStr<'re>,
  limit: u16,
  user_ids: Value
}

impl<'re> Default for Members<'re> {
  fn default() -> Self {
    Self {
      guild_id: 0,
      query: Cow::Borrowed(""),
      limit: 0,
      user_ids: Value::Null
    }
  }
}
