//
//

//!

use crate::models::prelude::*;

mod activity;
mod bot;
mod hello;
mod identify;
mod resume;
mod session_start_limit;
mod update_status;
mod voice_state_update;

pub use self::{
  activity::{ Activity, ActivityBuilder },
  bot::Bot,
  hello::Hello,
  identify::{
    Identify,
    IdentifyBuilder,
    ConnectionProperties,
    ConnectionPropertiesBuilder
  },
  resume::{ Resume, ResumeBuilder },
  session_start_limit::SessionStartLimit,
  update_status::{ UpdateStatus, UpdateStatusBuilder },
  voice_state_update::{ VoiceStateUpdate, VoiceStateUpdateBuilder }
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Gateway {
  #[serde(with = "crate::util::url")]
  url: Url
}
