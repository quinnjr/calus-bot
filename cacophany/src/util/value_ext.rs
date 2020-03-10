//
//

//!

use flate2::read::ZlibDecoder;
use serde_json::{ json, Value };
use shrinkwraprs::Shrinkwrap;

use std::{
  convert::{ From, TryFrom },
  fmt::{ self, Debug, Display },
  ops::{ Deref, DerefMut }
};

use crate::result::Result;

use super::MessageExt;

#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct ValueExt(pub serde_json::Value);

impl From<serde_json::Value> for ValueExt {
  fn from(val: serde_json::Value) -> Self {
    Self(val)
  }
}

impl From<ValueExt> for serde_json::Value {
  fn from(val: ValueExt) -> Self {
    val.0
  }
}

impl Display for ValueExt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<MessageExt> for ValueExt {
  type Error = crate::error::Error;

  fn try_from(message: MessageExt) -> Result<Self> {
    use tungstenite::Message::*;
    let val = match *message {
      Binary(bytes) => {
        let reader = ZlibDecoder::new(&bytes[..]);
        serde_json::from_reader(reader)?
      },
      Text(msg) => {
        json!(msg)
      },
      _ => Value::Null
    };
    Ok(ValueExt::from(val))
  }
}
