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
pub struct DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
  /// The activity for which this challenge is active.
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  #[serde(rename = "challenges")]
  challenges: Option<Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition>>,
  /// If the activity and its challenge is visible on any of these nodes, it will be returned.
  #[serde(rename = "activityGraphNodes")]
  activity_graph_nodes: Option<Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityGraphNodeEntry>>,
  /// Phases related to this activity, if there are any.  These will be listed in the order in which they will appear in the actual activity.
  #[serde(rename = "phases")]
  phases: Option<Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityPhase>>
}

impl DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
    DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
      activity_hash: None,
      challenges: None,
      activity_graph_nodes: None,
      phases: None
    }
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_challenges(&mut self, challenges: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition>) {
    self.challenges = Some(challenges);
  }

  pub fn with_challenges(mut self, challenges: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
    self.challenges = Some(challenges);
    self
  }

  pub fn challenges(&self) -> Option<&Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition>> {
    self.challenges.as_ref()
  }

  pub fn reset_challenges(&mut self) {
    self.challenges = None;
  }

  pub fn set_activity_graph_nodes(&mut self, activity_graph_nodes: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityGraphNodeEntry>) {
    self.activity_graph_nodes = Some(activity_graph_nodes);
  }

  pub fn with_activity_graph_nodes(mut self, activity_graph_nodes: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityGraphNodeEntry>) -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
    self.activity_graph_nodes = Some(activity_graph_nodes);
    self
  }

  pub fn activity_graph_nodes(&self) -> Option<&Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityGraphNodeEntry>> {
    self.activity_graph_nodes.as_ref()
  }

  pub fn reset_activity_graph_nodes(&mut self) {
    self.activity_graph_nodes = None;
  }

  pub fn set_phases(&mut self, phases: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityPhase>) {
    self.phases = Some(phases);
  }

  pub fn with_phases(mut self, phases: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityPhase>) -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition {
    self.phases = Some(phases);
    self
  }

  pub fn phases(&self) -> Option<&Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityPhase>> {
    self.phases.as_ref()
  }

  pub fn reset_phases(&mut self) {
    self.phases = None;
  }

}



