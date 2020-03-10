//
//

//!
//!
use crate::prelude::internal::*;

#[derive(Copy, Clone, Debug, PartialEq, Fail)]
pub struct ClientError;

impl Display for ClientError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
