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
pub struct DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition {
  /// The challenge related to this milestone.
  #[serde(rename = "challengeObjectiveHash")]
  challenge_objective_hash: Option<i32>
}

impl DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition {
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition {
    DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition {
      challenge_objective_hash: None
    }
  }

  pub fn set_challenge_objective_hash(&mut self, challenge_objective_hash: i32) {
    self.challenge_objective_hash = Some(challenge_objective_hash);
  }

  pub fn with_challenge_objective_hash(mut self, challenge_objective_hash: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition {
    self.challenge_objective_hash = Some(challenge_objective_hash);
    self
  }

  pub fn challenge_objective_hash(&self) -> Option<&i32> {
    self.challenge_objective_hash.as_ref()
  }

  pub fn reset_challenge_objective_hash(&mut self) {
    self.challenge_objective_hash = None;
  }

}



