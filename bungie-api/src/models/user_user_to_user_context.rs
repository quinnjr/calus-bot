/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUserToUserContext {
  #[serde(rename = "isFollowing")]
  is_following: Option<bool>,
  #[serde(rename = "ignoreStatus")]
  ignore_status: Option<::models::IgnoresIgnoreResponse>,
  #[serde(rename = "globalIgnoreEndDate")]
  global_ignore_end_date: Option<String>
}

impl UserUserToUserContext {
  pub fn new() -> UserUserToUserContext {
    UserUserToUserContext {
      is_following: None,
      ignore_status: None,
      global_ignore_end_date: None
    }
  }

  pub fn set_is_following(&mut self, is_following: bool) {
    self.is_following = Some(is_following);
  }

  pub fn with_is_following(mut self, is_following: bool) -> UserUserToUserContext {
    self.is_following = Some(is_following);
    self
  }

  pub fn is_following(&self) -> Option<&bool> {
    self.is_following.as_ref()
  }

  pub fn reset_is_following(&mut self) {
    self.is_following = None;
  }

  pub fn set_ignore_status(&mut self, ignore_status: ::models::IgnoresIgnoreResponse) {
    self.ignore_status = Some(ignore_status);
  }

  pub fn with_ignore_status(mut self, ignore_status: ::models::IgnoresIgnoreResponse) -> UserUserToUserContext {
    self.ignore_status = Some(ignore_status);
    self
  }

  pub fn ignore_status(&self) -> Option<&::models::IgnoresIgnoreResponse> {
    self.ignore_status.as_ref()
  }

  pub fn reset_ignore_status(&mut self) {
    self.ignore_status = None;
  }

  pub fn set_global_ignore_end_date(&mut self, global_ignore_end_date: String) {
    self.global_ignore_end_date = Some(global_ignore_end_date);
  }

  pub fn with_global_ignore_end_date(mut self, global_ignore_end_date: String) -> UserUserToUserContext {
    self.global_ignore_end_date = Some(global_ignore_end_date);
    self
  }

  pub fn global_ignore_end_date(&self) -> Option<&String> {
    self.global_ignore_end_date.as_ref()
  }

  pub fn reset_global_ignore_end_date(&mut self) {
    self.global_ignore_end_date = None;
  }

}



