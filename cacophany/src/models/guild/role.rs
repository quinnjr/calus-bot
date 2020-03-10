//
//

use serde::{ Deserialize, Serialize };

use crate::{
  models::Snowflake,
  util::CowStr
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize, Serialize)]
pub struct Role<'re> {
  id: Snowflake,
  name: CowStr<'re>,
  color: u32,
  hoist: bool,
  position: u32,
  permissions: u32,
  managed: bool,
  mentionable: bool
}

impl<'re> Default for Role<'re> {
  fn default() -> Self {
    Self {

    }
  }
}
