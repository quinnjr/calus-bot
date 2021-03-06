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
pub struct GroupsV2GroupResponse {
  #[serde(rename = "detail")]
  detail: Option<::models::GroupsV2GroupV2>,
  #[serde(rename = "founder")]
  founder: Option<::models::GroupsV2GroupMember>,
  #[serde(rename = "alliedIds")]
  allied_ids: Option<Vec<i64>>,
  #[serde(rename = "parentGroup")]
  parent_group: Option<::models::GroupsV2GroupV2>,
  #[serde(rename = "allianceStatus")]
  alliance_status: Option<i32>,
  #[serde(rename = "groupJoinInviteCount")]
  group_join_invite_count: Option<i32>,
  /// A convenience property that indicates if every membership you (the current user) have that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
  #[serde(rename = "currentUserMembershipsInactiveForDestiny")]
  current_user_memberships_inactive_for_destiny: Option<bool>,
  /// This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available.
  #[serde(rename = "currentUserMemberMap")]
  current_user_member_map: Option<::std::collections::HashMap<String, ::models::GroupsV2GroupMember>>,
  /// This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once.
  #[serde(rename = "currentUserPotentialMemberMap")]
  current_user_potential_member_map: Option<::std::collections::HashMap<String, ::models::GroupsV2GroupPotentialMember>>
}

impl GroupsV2GroupResponse {
  pub fn new() -> GroupsV2GroupResponse {
    GroupsV2GroupResponse {
      detail: None,
      founder: None,
      allied_ids: None,
      parent_group: None,
      alliance_status: None,
      group_join_invite_count: None,
      current_user_memberships_inactive_for_destiny: None,
      current_user_member_map: None,
      current_user_potential_member_map: None
    }
  }

  pub fn set_detail(&mut self, detail: ::models::GroupsV2GroupV2) {
    self.detail = Some(detail);
  }

  pub fn with_detail(mut self, detail: ::models::GroupsV2GroupV2) -> GroupsV2GroupResponse {
    self.detail = Some(detail);
    self
  }

  pub fn detail(&self) -> Option<&::models::GroupsV2GroupV2> {
    self.detail.as_ref()
  }

  pub fn reset_detail(&mut self) {
    self.detail = None;
  }

  pub fn set_founder(&mut self, founder: ::models::GroupsV2GroupMember) {
    self.founder = Some(founder);
  }

  pub fn with_founder(mut self, founder: ::models::GroupsV2GroupMember) -> GroupsV2GroupResponse {
    self.founder = Some(founder);
    self
  }

  pub fn founder(&self) -> Option<&::models::GroupsV2GroupMember> {
    self.founder.as_ref()
  }

  pub fn reset_founder(&mut self) {
    self.founder = None;
  }

  pub fn set_allied_ids(&mut self, allied_ids: Vec<i64>) {
    self.allied_ids = Some(allied_ids);
  }

  pub fn with_allied_ids(mut self, allied_ids: Vec<i64>) -> GroupsV2GroupResponse {
    self.allied_ids = Some(allied_ids);
    self
  }

  pub fn allied_ids(&self) -> Option<&Vec<i64>> {
    self.allied_ids.as_ref()
  }

  pub fn reset_allied_ids(&mut self) {
    self.allied_ids = None;
  }

  pub fn set_parent_group(&mut self, parent_group: ::models::GroupsV2GroupV2) {
    self.parent_group = Some(parent_group);
  }

  pub fn with_parent_group(mut self, parent_group: ::models::GroupsV2GroupV2) -> GroupsV2GroupResponse {
    self.parent_group = Some(parent_group);
    self
  }

  pub fn parent_group(&self) -> Option<&::models::GroupsV2GroupV2> {
    self.parent_group.as_ref()
  }

  pub fn reset_parent_group(&mut self) {
    self.parent_group = None;
  }

  pub fn set_alliance_status(&mut self, alliance_status: i32) {
    self.alliance_status = Some(alliance_status);
  }

  pub fn with_alliance_status(mut self, alliance_status: i32) -> GroupsV2GroupResponse {
    self.alliance_status = Some(alliance_status);
    self
  }

  pub fn alliance_status(&self) -> Option<&i32> {
    self.alliance_status.as_ref()
  }

  pub fn reset_alliance_status(&mut self) {
    self.alliance_status = None;
  }

  pub fn set_group_join_invite_count(&mut self, group_join_invite_count: i32) {
    self.group_join_invite_count = Some(group_join_invite_count);
  }

  pub fn with_group_join_invite_count(mut self, group_join_invite_count: i32) -> GroupsV2GroupResponse {
    self.group_join_invite_count = Some(group_join_invite_count);
    self
  }

  pub fn group_join_invite_count(&self) -> Option<&i32> {
    self.group_join_invite_count.as_ref()
  }

  pub fn reset_group_join_invite_count(&mut self) {
    self.group_join_invite_count = None;
  }

  pub fn set_current_user_memberships_inactive_for_destiny(&mut self, current_user_memberships_inactive_for_destiny: bool) {
    self.current_user_memberships_inactive_for_destiny = Some(current_user_memberships_inactive_for_destiny);
  }

  pub fn with_current_user_memberships_inactive_for_destiny(mut self, current_user_memberships_inactive_for_destiny: bool) -> GroupsV2GroupResponse {
    self.current_user_memberships_inactive_for_destiny = Some(current_user_memberships_inactive_for_destiny);
    self
  }

  pub fn current_user_memberships_inactive_for_destiny(&self) -> Option<&bool> {
    self.current_user_memberships_inactive_for_destiny.as_ref()
  }

  pub fn reset_current_user_memberships_inactive_for_destiny(&mut self) {
    self.current_user_memberships_inactive_for_destiny = None;
  }

  pub fn set_current_user_member_map(&mut self, current_user_member_map: ::std::collections::HashMap<String, ::models::GroupsV2GroupMember>) {
    self.current_user_member_map = Some(current_user_member_map);
  }

  pub fn with_current_user_member_map(mut self, current_user_member_map: ::std::collections::HashMap<String, ::models::GroupsV2GroupMember>) -> GroupsV2GroupResponse {
    self.current_user_member_map = Some(current_user_member_map);
    self
  }

  pub fn current_user_member_map(&self) -> Option<&::std::collections::HashMap<String, ::models::GroupsV2GroupMember>> {
    self.current_user_member_map.as_ref()
  }

  pub fn reset_current_user_member_map(&mut self) {
    self.current_user_member_map = None;
  }

  pub fn set_current_user_potential_member_map(&mut self, current_user_potential_member_map: ::std::collections::HashMap<String, ::models::GroupsV2GroupPotentialMember>) {
    self.current_user_potential_member_map = Some(current_user_potential_member_map);
  }

  pub fn with_current_user_potential_member_map(mut self, current_user_potential_member_map: ::std::collections::HashMap<String, ::models::GroupsV2GroupPotentialMember>) -> GroupsV2GroupResponse {
    self.current_user_potential_member_map = Some(current_user_potential_member_map);
    self
  }

  pub fn current_user_potential_member_map(&self) -> Option<&::std::collections::HashMap<String, ::models::GroupsV2GroupPotentialMember>> {
    self.current_user_potential_member_map.as_ref()
  }

  pub fn reset_current_user_potential_member_map(&mut self) {
    self.current_user_potential_member_map = None;
  }

}



