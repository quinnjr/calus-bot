//
//

//!

use crate::models::prelude::*;

use super::SessionStartLimit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bot {
  /// The WSS URL that can be used for connecting to the gateway
  #[serde(with = "crate::util::url")]
  url: Url,
  /// The recommended number of shards to use when connecting
  shards: isize,
  /// Information on the current session start limit
  session_start_limit: SessionStartLimit
}
