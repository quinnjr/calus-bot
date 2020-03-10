//
//

//!

use shrinkwraprs::Shrinkwrap;

use std::{
  convert::{ From, TryFrom },
  fmt::{ self, Debug, Display },
  ops::{ Deref, DerefMut }
};

use crate::result::Result;

use super::ValueExt;

#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct MessageExt(pub tungstenite::Message);

impl From<tungstenite::Message> for MessageExt {
  fn from(message: tungstenite::Message) -> Self {
    Self(message)
  }
}

impl From<MessageExt> for tungstenite::Message {
  fn from(message: MessageExt) -> Self {
    message.0
  }
}

impl Display for MessageExt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<ValueExt> for MessageExt {
  type Error = crate::error::Error;

  fn try_from(val: ValueExt) -> Result<MessageExt> {
    use serde_json::Value::*;
    use tungstenite::Message;
    // let val = *val;
    let msg = match *val {
      Array(val) => {
        Message::from(serde_json::to_vec(&val)?)
      },
      Bool(val) => Message::from(match val {
        true => "true",
        false => "false"
      }),
      Number(val) => {
        let val = val.as_i64().unwrap();
        let mut buf = Vec::new();
        itoa::write(&mut buf, val)?;
        Message::from(buf)
      },
      String(val) => Message::from(val),
      Object(obj) => {
        Message::from(serde_json::to_vec(&obj)?)
      },
      Null => Message::from(&[] as &[u8]),
      _ => unreachable!()
    };
    Ok(MessageExt::from(msg))
  }
}
