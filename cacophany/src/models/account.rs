//
//

use crate::models::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Account<'a> {
  _p: PhantomData<&'a str>
}
