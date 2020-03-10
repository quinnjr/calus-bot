//
//

//!

use std::{
  convert::{ From, TryFrom, TryInto },
  fmt::{ self, Display },
  ops::{ Deref, DerefMut }
};

pub(crate) mod duration;
pub(crate) mod url;

pub(crate) type CowStr<'lt> = ::std::borrow::Cow<'lt, str>;

mod message_ext;
pub use message_ext::MessageExt;

mod value_ext;
pub use value_ext::ValueExt;
