//
//

use crate::prelude::internal::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum Status {
  #[serde(rename = "online")]
  Online,
  #[serde(rename = "dnd")]
  DoNotDisturb,
  #[serde(rename = "idle")]
  Idle,
  #[serde(rename = "invisible")]
  Invisible,
  #[serde(rename = "offline")]
  Offline
}

impl<'lt> From<Status> for Cow<'lt, str> {
  fn from(status: Status) -> Self {
    use self::Status::*;

    match status {
      Online => "Online".into(),
      DoNotDisturb => "Do Not Disturb".into(),
      Idle => "Idle".into(),
      Invisible => "Invisible".into(),
      Offline => "Offline".into()
    }
  }
}

impl Default for Status {
  fn default() -> Self { Self::Online }
}
