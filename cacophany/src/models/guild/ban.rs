//
//

//!

use crate::models::{
  prelude::*,
  user::User
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Ban<'re> {
  reason: CowStr<'re>,
  user: User<'re>
}
