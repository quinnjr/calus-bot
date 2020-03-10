/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityChallengeDefinition : Represents a reference to a Challenge, which for now is just an Objective.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityChallengeDefinition {
  /// The hash for the Objective that matches this challenge. Use it to look up the DestinyObjectiveDefinition.
  #[serde(rename = "objectiveHash")]
  objective_hash: Option<i32>,
  /// The rewards as they're represented in the UI. Note that they generally link to \"dummy\" items that give a summary of rewards rather than direct, real items themselves.  If the quantity is 0, don't show the quantity.
  #[serde(rename = "dummyRewards")]
  dummy_rewards: Option<Vec<::models::DestinyDestinyItemQuantity>>
}

impl DestinyDefinitionsDestinyActivityChallengeDefinition {
  /// Represents a reference to a Challenge, which for now is just an Objective.
  pub fn new() -> DestinyDefinitionsDestinyActivityChallengeDefinition {
    DestinyDefinitionsDestinyActivityChallengeDefinition {
      objective_hash: None,
      dummy_rewards: None
    }
  }

  pub fn set_objective_hash(&mut self, objective_hash: i32) {
    self.objective_hash = Some(objective_hash);
  }

  pub fn with_objective_hash(mut self, objective_hash: i32) -> DestinyDefinitionsDestinyActivityChallengeDefinition {
    self.objective_hash = Some(objective_hash);
    self
  }

  pub fn objective_hash(&self) -> Option<&i32> {
    self.objective_hash.as_ref()
  }

  pub fn reset_objective_hash(&mut self) {
    self.objective_hash = None;
  }

  pub fn set_dummy_rewards(&mut self, dummy_rewards: Vec<::models::DestinyDestinyItemQuantity>) {
    self.dummy_rewards = Some(dummy_rewards);
  }

  pub fn with_dummy_rewards(mut self, dummy_rewards: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyDefinitionsDestinyActivityChallengeDefinition {
    self.dummy_rewards = Some(dummy_rewards);
    self
  }

  pub fn dummy_rewards(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.dummy_rewards.as_ref()
  }

  pub fn reset_dummy_rewards(&mut self) {
    self.dummy_rewards = None;
  }

}



