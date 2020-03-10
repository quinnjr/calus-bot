//
//

//!

use url::Url;
use serde::{ de, Deserializer, Serializer };

use std::fmt;

pub(crate) fn serialize<'a, S>(url: &Url, serializer: S)
  -> Result<S::Ok, S::Error>
  where S: Serializer
{
  serializer.serialize_str(url.as_str())
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
  where D: Deserializer<'de>
{
  deserializer.deserialize_str(UrlVisitor)
}

pub(crate) struct UrlVisitor;

impl<'de> de::Visitor<'de> for UrlVisitor {
  type Value = Url;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("a valid Url &str or String")
  }

  fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
    where E: de::Error
  {
    Url::parse(val)
      .map_err(|e| E::custom(e))
  }

  fn visit_string<E>(self, val: String) -> Result<Self::Value, E>
    where E: de::Error
  {
    Url::parse(val.as_str())
      .map_err(|e| E::custom(e))
  }
}
