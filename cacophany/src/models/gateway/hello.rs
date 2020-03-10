//
//

//!

use crate::models::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct Hello(#[serde(with = "crate::util::duration")] Duration);

impl Hello {
  fn set_duration(&mut self, dur: Duration) -> &mut Self {
    self.0 = dur;
    self
  }

  fn duration(&self) -> Duration {
    self.0
  }
}

impl Serialize for Hello {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok,
    S::Error> where S: Serializer
  {
    let mut state = serializer.serialize_struct("Hello", 1)?;
    state.serialize_field("heartbeat_interval", &self.0.num_milliseconds())?;
    state.end()
  }
}

impl Default for Hello {
  fn default() -> Self {
    Self(Duration::zero())
  }
}
