//
//

//!

#[doc(no_inline)]
use failure::{ err_msg, Fail };
#[doc(no_inline)]
use serde::{ Serialize, Deserialize };

use std::{
  fmt::{ self, Display }
};

macro_rules! error_enumeration {
  ( $( $Name:tt => $Cause:path; )+ ) => {
    #[derive(Debug, Fail)]
    pub enum Error {
      $(
        $Name(#[fail(cause)] $Cause),
      )+
    }

    impl Display for Error {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
      }
    }

    $(
      impl From<$Cause> for Error {
        fn from(err: $Cause) -> Self { Self::$Name(err) }
      }
    )+
  };
}

#[derive(Clone, Debug, PartialEq, Fail)]
#[fail(display = "Error recieved from the Discord API: {}.", meaning)]
pub struct DiscordJsonError {
  code: u16,
  meaning: String
}

error_enumeration! {
  Json => serde_json::Error;
  Tungstenite => tungstenite::error::Error;
  Format => std::fmt::Error;
  Io => std::io::Error;
  ParseInt => std::num::ParseIntError;
  Http => http::Error;
  H2 => h2::Error;
  Flate2Decompress => flate2::DecompressError;
  Flate2Compress => flate2::CompressError;
  TimeOutOfRange => time::OutOfRangeError;
  ChronoParse => chrono::ParseError;
  HttpToStr => http::header::ToStrError;
  TryFromInt => std::num::TryFromIntError;
  DiscordJson => self::DiscordJsonError;
  Fail => failure::Error;
}

impl From<String> for Error {
  fn from(message: String) -> Self {
    Error::Fail(err_msg(message))
  }
}
