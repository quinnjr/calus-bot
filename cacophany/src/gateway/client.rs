//
//

//!
//!

use chrono::{ Utc, Duration };
use flate2::read::ZlibDecoder;
use log::debug;
use serde_json::{ json, Value };
use tungstenite::{ WebSocket, Message, util::NonBlockingResult };
use rustls::{ StreamOwned, ClientSession };
use serde::Serialize;

use std::{
  borrow::Cow,
  convert::{ TryFrom, TryInto },
  env::consts as StdConsts,
  net::TcpStream
};

use crate::{
  consts,
  models::{
    guild::{
      Members,
      MembersBuilder,
      RequestMember,
      RequestMemberBuilder
    },
    Presence,
    Payload,
    PayloadBuilder,
    Snowflake
  },
  result::Result,
  util::{ CowStr, MessageExt, ValueExt }
};

use super::OpCode;

pub type Client = WebSocket<StreamOwned<ClientSession, TcpStream>>;

pub trait ClientExt {
  fn recieve(&mut self) -> Result<ValueExt>;

  fn try_recieve(&mut self) -> Result<ValueExt>;

  fn send<'re, P: Sized>(&mut self, data: &P,
    query: Option<CowStr<'re>>, shard_info: &[u32; 2],
    limit: Option<u16>) -> Result<()>
    where P: Serialize + Default;

  // fn send_chunk_guilds<'re, I>(&mut self, guild_ids: I,
  //   shard_info: &[usize; 2], limit: Option<u16>,
  //   query: Option<CowStr<'re>>) -> Result<()>
  //   where I: IntoIterator<Item=Snowflake>;

  fn send_heartbeat(&mut self, shard_info: &[u32; 2],
    seq: Option<usize>) -> Result<()>;

  fn send_identify<'re>(&mut self, shard_info: &[u32; 2],
    token: CowStr<'re>) -> Result<()>;

  fn send_presence(&mut self, shard_info: &[u32; 2],
    current_presence: &Presence) -> Result<()>;

  fn send_resume<'re>(&mut self, shard_info: &[u32; 2],
    session_id: CowStr<'re>, seq: usize, token: CowStr<'re>)
    -> Result<()>;
}

#[cfg(debug_assertions)]
fn debug_msg(shard_info: &[usize; 2], op_code: &OpCode) {
  debug!("[Shard {:?}] Sending {}...", shard_info, op_code);
}

impl ClientExt for Client {
  fn recieve(&mut self) -> Result<ValueExt> {
    let message: MessageExt = self.read_message()?.into();
    message.try_into()
  }

  fn try_recieve(&mut self) -> Result<ValueExt> {
    if let Some(message) = self.read_message().no_block()? {
      let message: MessageExt = message.into();
      Ok(message.try_into()?)
    } else {
      Ok(ValueExt::from(Value::Null))
    }
  }

  fn send<'re, P: Sized>(&mut self, payload: &P,
    query: Option<CowStr<'re>>, shard_info: &[u32; 2],
    limit: Option<u16>) -> Result<()>
    where P: Serialize + Default
  {
    serde_json::to_string(payload)
      .map(Message::Text)
      .map_err(crate::error::Error::Json)
      .and_then(|msg| {
        self.write_message(msg)
          .map_err(crate::error::Error::Tungstenite)
      })
  }

  // fn send_chunk_guilds<'re, I>(&mut self, guild_ids: I,
  //   shard_info: &[usize; 2], limit: Option<u16>,
  //   query: Option<CowStr<'re>>) -> Result<()>
  //   where I: IntoIterator<Item=Snowflake>
  // {
  //   let data = MembersBuilder::default()
  //     .guild_id(guild_ids.into_iter()
  //       .collect::<Vec<Snowflake>>()
  //     )
  //     .limit(limit.unwrap_or(0))
  //     .query(query.unwrap_or(Cow::Borrowed("")))
  //     .build()?;
  //
  //   let payload: Payload = PayloadBuilder::default()
  //     .op(OpCode::RequestGuildMembers as u8)
  //     .d(json!(data))
  //     .build()?;
  //
  //   self.send(&payload, query, shard_info, limit)
  // }

  fn send_heartbeat(&mut self, shard_info: &[u32; 2],
    seq: Option<usize>) -> Result<()>
  {
    let payload = PayloadBuilder::default()
      .op(OpCode::Heartbeat as u8)
      .d(seq.unwrap_or(0).into())
      .build()?;

    self.send(&payload, None, shard_info, None)
  }

  fn send_identify<'re>(&mut self, shard_info: &[u32; 2],
    token: CowStr<'re>) -> Result<()>
  {
    let props = crate::models::gateway::ConnectionProperties::default();

    let ident = crate::models::gateway::IdentifyBuilder::default()
      .shard(shard_info)
      .token(token)
      .properties(props)
      .build()?;

    let payload: Payload = PayloadBuilder::default()
      .op(OpCode::Identify as u8)
      .d(json!(ident))
      .build()?;

    self.send(&payload, None, shard_info, None)
  }

  fn send_presence(&mut self, shard_info: &[u32; 2],
    current_presence: &Presence) -> Result<()>
  {
    let (activity, status) = *current_presence;
    let now = Utc::now().timestamp() * 1000;

    let presence = crate::models::gateway::UpdateStatusBuilder::default()
      .since(Some(Duration::milliseconds(now)))
      .status(status)
      .game(activity)
      .build()?;

    let payload: Payload = PayloadBuilder::default()
      .op(OpCode::StatusUpdate as u8)
      .d(json!(presence))
      .build()?;

    self.send(&payload, None, shard_info, None)
  }

  fn send_resume<'re>(&mut self, shard_info: &[u32; 2],
    session_id: CowStr<'re>, seq: usize,
    token: CowStr<'re>) -> Result<()>
  {
    let resume = crate::models::gateway::ResumeBuilder::default()
      .session_id(session_id)
      .seq(seq.try_into()?)
      .token(token)
      .build();

    let payload: Payload = PayloadBuilder::default()
      .op(OpCode::Resume as u8)
      .d(json!(resume))
      .build()?;

    self.send(&payload, None, shard_info, None)
  }
}
