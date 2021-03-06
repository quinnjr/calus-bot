/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDestinyTalentNodeStatBlock : This property has some history. A talent grid can provide stats on both the item it's related to and the character equipping the item. This returns data about those stat bonuses.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDestinyTalentNodeStatBlock {
  /// The stat benefits conferred when this talent node is activated for the current Step that is active on the node.
  #[serde(rename = "currentStepStats")]
  current_step_stats: Option<Vec<::models::DestinyDestinyStat>>,
  /// This is a holdover from the old days of Destiny 1, when a node could be activated multiple times, conferring multiple steps worth of benefits: you would use this property to show what activating the \"next\" step on the node would provide vs. what the current step is providing. While Nodes are currently not being used this way, the underlying system for this functionality still exists. I hesitate to remove this property while the ability for designers to make such a talent grid still exists. Whether you want to show it is up to you.
  #[serde(rename = "nextStepStats")]
  next_step_stats: Option<Vec<::models::DestinyDestinyStat>>
}

impl DestinyDestinyTalentNodeStatBlock {
  /// This property has some history. A talent grid can provide stats on both the item it's related to and the character equipping the item. This returns data about those stat bonuses.
  pub fn new() -> DestinyDestinyTalentNodeStatBlock {
    DestinyDestinyTalentNodeStatBlock {
      current_step_stats: None,
      next_step_stats: None
    }
  }

  pub fn set_current_step_stats(&mut self, current_step_stats: Vec<::models::DestinyDestinyStat>) {
    self.current_step_stats = Some(current_step_stats);
  }

  pub fn with_current_step_stats(mut self, current_step_stats: Vec<::models::DestinyDestinyStat>) -> DestinyDestinyTalentNodeStatBlock {
    self.current_step_stats = Some(current_step_stats);
    self
  }

  pub fn current_step_stats(&self) -> Option<&Vec<::models::DestinyDestinyStat>> {
    self.current_step_stats.as_ref()
  }

  pub fn reset_current_step_stats(&mut self) {
    self.current_step_stats = None;
  }

  pub fn set_next_step_stats(&mut self, next_step_stats: Vec<::models::DestinyDestinyStat>) {
    self.next_step_stats = Some(next_step_stats);
  }

  pub fn with_next_step_stats(mut self, next_step_stats: Vec<::models::DestinyDestinyStat>) -> DestinyDestinyTalentNodeStatBlock {
    self.next_step_stats = Some(next_step_stats);
    self
  }

  pub fn next_step_stats(&self) -> Option<&Vec<::models::DestinyDestinyStat>> {
    self.next_step_stats.as_ref()
  }

  pub fn reset_next_step_stats(&mut self) {
    self.next_step_stats = None;
  }

}



