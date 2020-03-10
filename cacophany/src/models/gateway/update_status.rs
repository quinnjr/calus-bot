//
//

use crate::models::{
  prelude::*,
  user::Status
};

use super::Activity;

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
pub struct UpdateStatus<'re> {
  #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "self::deserialize", serialize_with = "self::serialize")]
  since: Option<Duration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  game: Option<Activity<'re>>,
  #[serde(default)]
  status: Status,
  afk: bool
}

impl<'re> Default for UpdateStatus<'re> {
  fn default() -> Self {
    Self {
      since: None,
      game: None,
      status: Status::Online,
      afk: false
    }
  }
}

fn serialize<S>(duration: &Option<Duration>, serializer: S) -> StdResult<S::Ok, S::Error>
  where S: Serializer
{
  match duration {
    Some(dur) => serializer.serialize_i64(dur.num_seconds()),
    // None should be unreachable since we skip serializing
    // if the field is an Option::None
    None => unreachable!(),
  }
}

fn deserialize<'de, D>(deserializer: D)
  -> StdResult<Option<Duration>, D::Error>
  where D: Deserializer<'de>
{
  let duration = crate::util::duration::deserialize(deserializer)?;
  if duration.is_zero() {
    Ok(None)
  } else {
    Ok(Some(duration))
  }
}
