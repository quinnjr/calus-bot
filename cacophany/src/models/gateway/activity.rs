//
//

use crate::models::prelude::*;

#[derive(Clone, Debug, PartialEq, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Activity<'re> {
  name: CowStr<'re>,
  #[serde(rename = "type")]
  ty: CowStr<'re>,
  #[serde(with = "crate::util::url")]
  url: Url
}

impl<'re> Default for Activity<'re> {
  fn default() -> Self {
    Self {
      name: CowStr::Borrowed(""),
      ty: CowStr::Borrowed(""),
      url: Url::parse("").unwrap()
    }
  }
}
