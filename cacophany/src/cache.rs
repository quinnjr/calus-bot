//
//

//!
//!
//!

use crate::prelude::internal::*;

///
pub trait Cache {
  fn connect<Co>(self) -> Result<Co>;
  fn disconnect<Di>(&self) -> Result<Di>;
  fn get<'re>(&self, key: impl Into<CowStr<'re>>) -> Result<Value>;
  fn set<'re, S>(&self, key: impl Into<CowStr<'re>>, val: Value) -> Result<S>;
  fn delete<'re, D>(&self, key: impl Into<CowStr<'re>>) -> Result<D>;
  fn contains<'re, C>(&self, key: impl Into<CowStr<'re>>) -> Result<C>;
  fn is_connect(&self) -> Result<bool>;
}
