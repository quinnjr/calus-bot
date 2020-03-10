//
//

//!

use chrono::Duration;
use serde::{ self, Deserialize, Serializer, Deserializer };
use std::time::Duration as StdDuration;

pub fn serialize<S>(duration: &Duration, serializer: S)
  -> std::result::Result<S::Ok, S::Error>
  where S: Serializer
{
  serializer.serialize_i64(duration.num_milliseconds())
}

pub fn deserialize<'de, D>(deserializer: D)
  -> std::result::Result<chrono::Duration, D::Error>
  where D: Deserializer<'de>
{
  let secs = i64::deserialize(deserializer)?;
  Ok(Duration::milliseconds(secs))
}
