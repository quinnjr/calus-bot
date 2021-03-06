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
pub struct TrendingTrendingEntryDestinyActivity {
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  #[serde(rename = "status")]
  status: Option<::models::DestinyActivitiesDestinyPublicActivityStatus>
}

impl TrendingTrendingEntryDestinyActivity {
  pub fn new() -> TrendingTrendingEntryDestinyActivity {
    TrendingTrendingEntryDestinyActivity {
      activity_hash: None,
      status: None
    }
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> TrendingTrendingEntryDestinyActivity {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_status(&mut self, status: ::models::DestinyActivitiesDestinyPublicActivityStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::DestinyActivitiesDestinyPublicActivityStatus) -> TrendingTrendingEntryDestinyActivity {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::DestinyActivitiesDestinyPublicActivityStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



