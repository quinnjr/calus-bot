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
pub struct GroupsV2GroupNameSearchRequest {
  #[serde(rename = "groupName")]
  group_name: Option<String>,
  #[serde(rename = "groupType")]
  group_type: Option<i32>
}

impl GroupsV2GroupNameSearchRequest {
  pub fn new() -> GroupsV2GroupNameSearchRequest {
    GroupsV2GroupNameSearchRequest {
      group_name: None,
      group_type: None
    }
  }

  pub fn set_group_name(&mut self, group_name: String) {
    self.group_name = Some(group_name);
  }

  pub fn with_group_name(mut self, group_name: String) -> GroupsV2GroupNameSearchRequest {
    self.group_name = Some(group_name);
    self
  }

  pub fn group_name(&self) -> Option<&String> {
    self.group_name.as_ref()
  }

  pub fn reset_group_name(&mut self) {
    self.group_name = None;
  }

  pub fn set_group_type(&mut self, group_type: i32) {
    self.group_type = Some(group_type);
  }

  pub fn with_group_type(mut self, group_type: i32) -> GroupsV2GroupNameSearchRequest {
    self.group_type = Some(group_type);
    self
  }

  pub fn group_type(&self) -> Option<&i32> {
    self.group_type.as_ref()
  }

  pub fn reset_group_type(&mut self) {
    self.group_type = None;
  }

}



