//
//

//!

use serde::{ Deserialize, Serialize };
use serde_repr::{ Deserialize_repr, Serialize_repr };

use std::{
  clone::Clone,
  cmp::{ Eq, Ord, PartialEq, PartialOrd },
  fmt::{ self, Debug, Display },
  marker::Copy
};

mod client;
pub use client::{ Client, ClientExt };

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum OpCode {
  Dispatch = 0,
  Heartbeat,
  Identify,
  StatusUpdate,
  VoiceStateUpdate,
  Resume = 6,
  Reconnect,
  RequestGuildMembers,
  InvalidSession,
  Hello,
  HeartbeatAck
}

impl Display for OpCode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::OpCode::*;

    write!(f, "{}", match *self {
      Dispatch => "dispatching",
      Heartbeat => "heartbeat",
      Identify => "identifying",
      StatusUpdate => "status update",
      VoiceStateUpdate => "voice state update",
      Resume => "resume",
      Reconnect => "reconnecting",
      RequestGuildMembers => "requesting guild members",
      InvalidSession => "invalid session",
      Hello => "hello!",
      HeartbeatAck => "heartbeat acknowledge",
      _ => unreachable!()
    })
  }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum CloseEvent {
  Unknown = 4000,
  UnknownOpCode = 4001,
  DecodeError = 4002,
  NotAuthenticated = 4003,
  AuthenticationFailed = 4004,
  AlreadyAuthenticated = 4005,
  InvalidSequence = 4007,
  RateLimited = 4008,
  SessionTimeout = 4009,
  InvalidShard = 4010,
  ShardingRequired = 4011
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VoiceOpCode {
  Identify,
  SelectProtocol,
  Ready,
  Heartbeat,
  SessionDescription,
  Speaking,
  HeartbeatAck,
  Resume,
  Hello,
  Resumed,
  ClientDisconnect
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum VoiceCloseEvent {
  UnknownOpCode = 4001,
  NotAuthenticated = 4003,
  AuthenticationFailed = 4004,
  AlreadyAuthenticated = 4005,
  SessionNoLongerValid = 4006,
  SessionTimeout = 4009,
  ServerNotFound = 4011,
  UnknownProtocol = 4012,
  Disconnected = 4014,
  VoiceServerCrash = 4015,
  UnknownEncryptionMode = 4016
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum ConnectionStatus {
  Disconnected = -1,
  Connecting,
  Connected,
  Handshake,
  Identifying,
  Resuming
}

impl ConnectionStatus {
  pub fn is_connecting(self) -> bool {
    use self::ConnectionStatus::*;

    match self {
      Connecting | Handshake | Identifying | Resuming => true,
      Connected | Disconnected => false,
      _ => unreachable!()
    }
  }
}

impl Display for ConnectionStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use self::ConnectionStatus::*;

    write!(f, "{}", match *self {
      Connected => "connected",
      Connecting => "connecting",
      Disconnected => "disconnected",
      Handshake => "handshaking",
      Identifying => "identifying",
      Resuming => "resuming",
      _ => unreachable!()
    })
  }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ReconnectKind {
  Reidentify,
  Resume
}

impl<'a> Into<&'a str> for ReconnectKind {
  fn into(self) -> &'a str {
    use self::ReconnectKind::*;
    match self {
      Reidentify => "reidentify",
      Resume => "resume",
      _ => unreachable!()
    }
  }
}

impl Display for ReconnectKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use self::ReconnectKind::*;

    write!(f, "{}", match *self {
      Reidentify => "reidentify",
      Resume => "resume",
      _ => unreachable!()
    })
  }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
  Deserialize, Serialize)]
pub enum ShardStatus {
  Heartbeat,
  Identify,
  Reconnect(ReconnectKind)
}

impl Display for ShardStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use self::ShardStatus::*;

    write!(f, "{}", match *self {
      Heartbeat => "heartbeat",
      Identify => "identify",
      Reconnect(kind) => &["reconnect: ", kind.into()].concat(),
      _ => unreachable!()
    })
  }
}
