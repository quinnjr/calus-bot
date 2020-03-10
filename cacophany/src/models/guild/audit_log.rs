//
//

//!

use crate::models::{
  audit_log::AuditLogEvent,
  prelude::*
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AuditLog {
  user_id: Snowflake,
  action_type: AuditLogEvent,
  before: Snowflake,
  #[serde(default)] limit: u8
}
