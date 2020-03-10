//
//

//!


use crate::{
  models::{
    account::Account,
    prelude::*,
    user::User
  },
  util::CowStr
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Integration<'re> {
  id: Snowflake,
  name: CowStr<'re>,
  #[serde(rename = "type")]
  ty: CowStr<'re>,
  enabled: bool,
  syncing: bool,
  role_id: Snowflake,
  expire_behavior: i32,
  expire_grace_period: i32,
  user: User<'re>,
  account: Account<'re>,
  synced_at: CowStr<'re>
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct IntegrationAccount<'re> {
  id: CowStr<'re>,
  name: CowStr<'re>
}
