//
//

//!

use crate::models::prelude::*;

#[derive(Copy, Clone, Debug, Builder, Deserialize, Serialize)]
pub struct VoiceStateUpdate {
  guild_id: Snowflake,
  channel_id: Option<Snowflake>,
  self_mute: bool,
  self_deaf: bool,
}
