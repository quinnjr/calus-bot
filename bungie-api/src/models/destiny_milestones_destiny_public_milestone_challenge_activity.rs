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
pub struct DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  #[serde(rename = "challengeObjectiveHashes")]
  challenge_objective_hashes: Option<Vec<i32>>,
  /// If the activity has modifiers, this will be the list of modifiers that all variants have in common. Perform lookups against DestinyActivityModifierDefinition which defines the modifier being applied to get at the modifier data.  Note that, in the DestiyActivityDefinition, you will see many more modifiers than this being referred to: those are all *possible* modifiers for the activity, not the active ones. Use only the active ones to match what's really live.
  #[serde(rename = "modifierHashes")]
  modifier_hashes: Option<Vec<i32>>,
  /// If returned, this is the index into the DestinyActivityDefinition's \"loadouts\" property, indicating the currently active loadout requirements.
  #[serde(rename = "loadoutRequirementIndex")]
  loadout_requirement_index: Option<i32>,
  /// The ordered list of phases for this activity, if any. Note that we have no human readable info for phases, nor any entities to relate them to: relating these hashes to something human readable is up to you unfortunately.
  #[serde(rename = "phaseHashes")]
  phase_hashes: Option<Vec<i32>>,
  /// The set of activity options for this activity, keyed by an identifier that's unique for this activity (not guaranteed to be unique between or across all activities, though should be unique for every *variant* of a given *conceptual* activity: for instance, the original D2 Raid has many variant DestinyActivityDefinitions. While other activities could potentially have the same option hashes, for any given D2 base Raid variant the hash will be unique).  As a concrete example of this data, the hashes you get for Raids will correspond to the currently active \"Challenge Mode\".  We have no human readable information for this data, so it's up to you if you want to associate it with such info to show it.
  #[serde(rename = "booleanActivityOptions")]
  boolean_activity_options: Option<::std::collections::HashMap<String, bool>>
}

impl DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
  pub fn new() -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
      activity_hash: None,
      challenge_objective_hashes: None,
      modifier_hashes: None,
      loadout_requirement_index: None,
      phase_hashes: None,
      boolean_activity_options: None
    }
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_challenge_objective_hashes(&mut self, challenge_objective_hashes: Vec<i32>) {
    self.challenge_objective_hashes = Some(challenge_objective_hashes);
  }

  pub fn with_challenge_objective_hashes(mut self, challenge_objective_hashes: Vec<i32>) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.challenge_objective_hashes = Some(challenge_objective_hashes);
    self
  }

  pub fn challenge_objective_hashes(&self) -> Option<&Vec<i32>> {
    self.challenge_objective_hashes.as_ref()
  }

  pub fn reset_challenge_objective_hashes(&mut self) {
    self.challenge_objective_hashes = None;
  }

  pub fn set_modifier_hashes(&mut self, modifier_hashes: Vec<i32>) {
    self.modifier_hashes = Some(modifier_hashes);
  }

  pub fn with_modifier_hashes(mut self, modifier_hashes: Vec<i32>) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.modifier_hashes = Some(modifier_hashes);
    self
  }

  pub fn modifier_hashes(&self) -> Option<&Vec<i32>> {
    self.modifier_hashes.as_ref()
  }

  pub fn reset_modifier_hashes(&mut self) {
    self.modifier_hashes = None;
  }

  pub fn set_loadout_requirement_index(&mut self, loadout_requirement_index: i32) {
    self.loadout_requirement_index = Some(loadout_requirement_index);
  }

  pub fn with_loadout_requirement_index(mut self, loadout_requirement_index: i32) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.loadout_requirement_index = Some(loadout_requirement_index);
    self
  }

  pub fn loadout_requirement_index(&self) -> Option<&i32> {
    self.loadout_requirement_index.as_ref()
  }

  pub fn reset_loadout_requirement_index(&mut self) {
    self.loadout_requirement_index = None;
  }

  pub fn set_phase_hashes(&mut self, phase_hashes: Vec<i32>) {
    self.phase_hashes = Some(phase_hashes);
  }

  pub fn with_phase_hashes(mut self, phase_hashes: Vec<i32>) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.phase_hashes = Some(phase_hashes);
    self
  }

  pub fn phase_hashes(&self) -> Option<&Vec<i32>> {
    self.phase_hashes.as_ref()
  }

  pub fn reset_phase_hashes(&mut self) {
    self.phase_hashes = None;
  }

  pub fn set_boolean_activity_options(&mut self, boolean_activity_options: ::std::collections::HashMap<String, bool>) {
    self.boolean_activity_options = Some(boolean_activity_options);
  }

  pub fn with_boolean_activity_options(mut self, boolean_activity_options: ::std::collections::HashMap<String, bool>) -> DestinyMilestonesDestinyPublicMilestoneChallengeActivity {
    self.boolean_activity_options = Some(boolean_activity_options);
    self
  }

  pub fn boolean_activity_options(&self) -> Option<&::std::collections::HashMap<String, bool>> {
    self.boolean_activity_options.as_ref()
  }

  pub fn reset_boolean_activity_options(&mut self) {
    self.boolean_activity_options = None;
  }

}



