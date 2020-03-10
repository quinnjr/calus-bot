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
pub struct DestinyHistoricalStatsDestinyAggregateActivityResults {
  /// List of all activities the player has participated in.
  #[serde(rename = "activities")]
  activities: Option<Vec<::models::DestinyHistoricalStatsDestinyAggregateActivityStats>>
}

impl DestinyHistoricalStatsDestinyAggregateActivityResults {
  pub fn new() -> DestinyHistoricalStatsDestinyAggregateActivityResults {
    DestinyHistoricalStatsDestinyAggregateActivityResults {
      activities: None
    }
  }

  pub fn set_activities(&mut self, activities: Vec<::models::DestinyHistoricalStatsDestinyAggregateActivityStats>) {
    self.activities = Some(activities);
  }

  pub fn with_activities(mut self, activities: Vec<::models::DestinyHistoricalStatsDestinyAggregateActivityStats>) -> DestinyHistoricalStatsDestinyAggregateActivityResults {
    self.activities = Some(activities);
    self
  }

  pub fn activities(&self) -> Option<&Vec<::models::DestinyHistoricalStatsDestinyAggregateActivityStats>> {
    self.activities.as_ref()
  }

  pub fn reset_activities(&mut self) {
    self.activities = None;
  }

}



