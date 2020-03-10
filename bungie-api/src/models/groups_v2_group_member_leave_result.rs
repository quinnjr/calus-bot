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
pub struct GroupsV2GroupMemberLeaveResult {
  #[serde(rename = "group")]
  group: Option<::models::GroupsV2GroupV2>,
  #[serde(rename = "groupDeleted")]
  group_deleted: Option<bool>
}

impl GroupsV2GroupMemberLeaveResult {
  pub fn new() -> GroupsV2GroupMemberLeaveResult {
    GroupsV2GroupMemberLeaveResult {
      group: None,
      group_deleted: None
    }
  }

  pub fn set_group(&mut self, group: ::models::GroupsV2GroupV2) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: ::models::GroupsV2GroupV2) -> GroupsV2GroupMemberLeaveResult {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&::models::GroupsV2GroupV2> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_group_deleted(&mut self, group_deleted: bool) {
    self.group_deleted = Some(group_deleted);
  }

  pub fn with_group_deleted(mut self, group_deleted: bool) -> GroupsV2GroupMemberLeaveResult {
    self.group_deleted = Some(group_deleted);
    self
  }

  pub fn group_deleted(&self) -> Option<&bool> {
    self.group_deleted.as_ref()
  }

  pub fn reset_group_deleted(&mut self) {
    self.group_deleted = None;
  }

}


