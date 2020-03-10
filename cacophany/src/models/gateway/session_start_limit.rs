//
//

//!

use crate::models::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SessionStartLimit {
  total: isize,
  remaining: isize,
  reset_after: isize
}
