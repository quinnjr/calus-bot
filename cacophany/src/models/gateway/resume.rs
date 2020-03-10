//
//

//!

use crate::models::prelude::*;

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
pub struct Resume<'re> {
  token: CowStr<'re>,
  session_id: CowStr<'re>,
  seq: u32
}

impl<'re> Default for Resume<'re> {
  fn default() -> Self {
    Self {
      token: CowStr::Borrowed(""),
      session_id: CowStr::Borrowed(""),
      seq: 0
    }
  }
}
