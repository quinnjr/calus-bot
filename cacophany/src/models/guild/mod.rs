//
//

//!

use chrono::{ DateTime, Duration, Utc };
use serde::{ Deserialize, Serialize };
use serde_json::Value;

use std::{
  borrow::Cow
};

use crate::{
  models::Snowflake,
  util::CowStr
};

use super::user::User;

mod ban;
mod integration;
mod member;
mod members;
mod request_member;

pub use self::{
  ban::Ban,
  integration::{ Integration, IntegrationAccount },
  member::{ Member, MemberBuilder },
  members::{ Members, MembersBuilder },
  request_member::{ RequestMember, RequestMemberBuilder }
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Guild<'re> {
  id: Snowflake,
  channel_id: Option<CowStr<'re>>,
  name: CowStr<'re>,
  icon: Option<CowStr<'re>>,
  splash: Option<CowStr<'re>>,
  owner: Option<bool>,
  permissions: Option<i32>,
  region: CowStr<'re>,
  afk_channel_id: Option<Snowflake>,
  #[serde(with = "crate::util::duration")]
  afk_timeout: Duration,
  embed_enabled: Option<bool>,
  embed_channel_id: Option<Snowflake>,
  verification_level: i32,
  default_message_notifications: i32,
  explicit_content_filter: i32,
  roles: Cow<'re, [Value]>,
  emojis: Cow<'re, [Value]>,
  features: Cow<'re, [Value]>,
  mfa_level: i32,
  application_id: Option<Snowflake>,
  widget_enabled: Option<bool>,
  widget_channel_id: Option<Snowflake>,
  system_channel_id: Option<Snowflake>,
  joined_at: Option<CowStr<'re>>,
  large: Option<bool>,
  unavailable: Option<bool>,
  member_count: Option<i32>,
  voice_states: Option<Cow<'re, [Value]>>,
  members: Option<Cow<'re, [Value]>>,
  channels: Option<Cow<'re, [Value]>>,
  presences: Option<Cow<'re, [Value]>>,
  max_presences: Option<i32>,
  max_members: Option<i32>,
  vanity_url_code: Option<CowStr<'re>>,
  description: Option<CowStr<'re>>,
  banner: Option<CowStr<'re>>,
  premium_tier: i32,
  premium_subscription_count: Option<i32>,
  preferred_locale: CowStr<'re>
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(C)]
pub enum DefaultMessageNotificationLevel {
  AllMessages,
  OnlyMentions
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(C)]
pub enum ExplicitContentFilterLevel {
  Disabled,
  MembersWithoutRoles,
  AllMembers
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(C)]
pub enum MFALevel {
  None,
  Elevated
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(C)]
pub enum VerificationLevel {
  None,
  Low,
  Medium,
  High,
  VeryHigh
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(C)]
pub enum PremiumTier {
  None,
  Tier1,
  Tier2,
  Tier3
}
