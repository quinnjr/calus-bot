//
//

//!

use chrono::{ DateTime, Utc };
use derive_builder::Builder;
use serde::{ Deserialize, Serialize };

use std::borrow::Cow;

use crate::{
  models::{
    Snowflake,
    user::User
  },
  util::CowStr
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Builder, Deserialize, Serialize)]
pub struct Member<'m> {
  user: User<'m>,
  nick: Option<CowStr<'m>>,
  roles: Option<Cow<'m, [Snowflake]>>,
  joined_at: DateTime<Utc>,
  premium_since: Option<CowStr<'m>>,
  deaf: bool,
  mute: bool
}
