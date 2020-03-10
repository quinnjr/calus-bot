//
//

//!

use crate::models::{
  prelude::*,
  user::User,
  Webhook
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogChange<'re> {
  new_value: Option<serde_json::Value>,
  old_value: Option<serde_json::Value>,
  key: CowStr<'re>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditEntryInfo<'re> {
  delete_member_days: Option<CowStr<'re>>,
  members_removed: Option<CowStr<'re>>,
  channel_id: Option<Snowflake>,
  count: Option<CowStr<'re>>,
  id: Option<Snowflake>,
  #[serde(rename = "type")]
  ty: Option<CowStr<'re>>,
  role_name: Option<CowStr<'re>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogEntry<'re> {
  target_id: Option<CowStr<'re>>,
  changes: Option<Cow<'re, [AuditLogChange<'re>]>>,
  user_id: Snowflake,
  id: Snowflake,
  action_type: AuditLogEvent,
  options: Option<AuditEntryInfo<'re>>,
  reason: Option<CowStr<'re>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLog<'re> {
  webhooks: Cow<'re, [Webhook]>,
  users: Cow<'re, [User<'re>]>,
  audit_log_entries: Cow<'re, [AuditLogEntry<'re>]>
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[repr(u8)]
pub enum AuditLogEvent {
  GuildUpdate = 1,
  ChannelCreate = 10,
  ChannelUpdate,
  ChannelDelete,
  ChannelOverwriteCreate,
  ChannelOverwriteUpdate,
  ChannelOverwriteDeletion,
  MemberKick = 20,
  MemberPrune,
  MemberBanAdd,
  MemberBanRemove,
  MemberUpdate,
  MemberRoleUpdate,
  RoleCreate = 30,
  RoleUpdate,
  RoleDelete,
  InviteCreate = 40,
  InviteUpdate,
  InviteDelete,
  WebhookCreate = 50,
  WebhookUpdate,
  WebhookDelete,
  EmojiCreate = 60,
  EmojiUpdate,
  EmojiDelete,
  MessageDelete = 72
}
