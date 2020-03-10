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
pub struct DestinyRequestsActionsDestinyActionRequest {
  #[serde(rename = "membershipType")]
  membership_type: Option<i32>
}

impl DestinyRequestsActionsDestinyActionRequest {
  pub fn new() -> DestinyRequestsActionsDestinyActionRequest {
    DestinyRequestsActionsDestinyActionRequest {
      membership_type: None
    }
  }

  pub fn set_membership_type(&mut self, membership_type: i32) {
    self.membership_type = Some(membership_type);
  }

  pub fn with_membership_type(mut self, membership_type: i32) -> DestinyRequestsActionsDestinyActionRequest {
    self.membership_type = Some(membership_type);
    self
  }

  pub fn membership_type(&self) -> Option<&i32> {
    self.membership_type.as_ref()
  }

  pub fn reset_membership_type(&mut self) {
    self.membership_type = None;
  }

}



