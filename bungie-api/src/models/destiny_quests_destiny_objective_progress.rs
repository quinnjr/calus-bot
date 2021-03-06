/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyQuestsDestinyObjectiveProgress : Returns data about a character's status with a given Objective. Combine with DestinyObjectiveDefinition static data for display purposes.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyQuestsDestinyObjectiveProgress {
  /// The unique identifier of the Objective being referred to. Use to look up the DestinyObjectiveDefinition in static data.
  #[serde(rename = "objectiveHash")]
  objective_hash: Option<i32>,
  /// If the Objective has a Destination associated with it, this is the unique identifier of the Destination being referred to. Use to look up the DestinyDestinationDefinition in static data. This will give localized data about *where* in the universe the objective should be achieved.
  #[serde(rename = "destinationHash")]
  destination_hash: Option<i32>,
  /// If the Objective has an Activity associated with it, this is the unique identifier of the Activity being referred to. Use to look up the DestinyActivityDefinition in static data. This will give localized data about *what* you should be playing for the objective to be achieved.
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  /// If progress has been made, and the progress can be measured numerically, this will be the value of that progress. You can compare it to the DestinyObjectiveDefinition.completionValue property for current vs. upper bounds, and use DestinyObjectiveDefinition.valueStyle to determine how this should be rendered. Note that progress, in Destiny 2, need not be a literal numeric progression. It could be one of a number of possible values, even a Timestamp. Always examine DestinyObjectiveDefinition.valueStyle before rendering progress.
  #[serde(rename = "progress")]
  progress: Option<i32>,
  /// As of Forsaken, objectives' completion value is determined dynamically at runtime.  This value represents the threshold of progress you need to surpass in order for this objective to be considered \"complete\".  If you were using objective data, switch from using the DestinyObjectiveDefinition's \"completionValue\" to this value.
  #[serde(rename = "completionValue")]
  completion_value: Option<i32>,
  /// Whether or not the Objective is completed.
  #[serde(rename = "complete")]
  complete: Option<bool>,
  /// If this is true, the objective is visible in-game. Otherwise, it's not yet visible to the player. Up to you if you want to honor this property.
  #[serde(rename = "visible")]
  visible: Option<bool>
}

impl DestinyQuestsDestinyObjectiveProgress {
  /// Returns data about a character's status with a given Objective. Combine with DestinyObjectiveDefinition static data for display purposes.
  pub fn new() -> DestinyQuestsDestinyObjectiveProgress {
    DestinyQuestsDestinyObjectiveProgress {
      objective_hash: None,
      destination_hash: None,
      activity_hash: None,
      progress: None,
      completion_value: None,
      complete: None,
      visible: None
    }
  }

  pub fn set_objective_hash(&mut self, objective_hash: i32) {
    self.objective_hash = Some(objective_hash);
  }

  pub fn with_objective_hash(mut self, objective_hash: i32) -> DestinyQuestsDestinyObjectiveProgress {
    self.objective_hash = Some(objective_hash);
    self
  }

  pub fn objective_hash(&self) -> Option<&i32> {
    self.objective_hash.as_ref()
  }

  pub fn reset_objective_hash(&mut self) {
    self.objective_hash = None;
  }

  pub fn set_destination_hash(&mut self, destination_hash: i32) {
    self.destination_hash = Some(destination_hash);
  }

  pub fn with_destination_hash(mut self, destination_hash: i32) -> DestinyQuestsDestinyObjectiveProgress {
    self.destination_hash = Some(destination_hash);
    self
  }

  pub fn destination_hash(&self) -> Option<&i32> {
    self.destination_hash.as_ref()
  }

  pub fn reset_destination_hash(&mut self) {
    self.destination_hash = None;
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyQuestsDestinyObjectiveProgress {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_progress(&mut self, progress: i32) {
    self.progress = Some(progress);
  }

  pub fn with_progress(mut self, progress: i32) -> DestinyQuestsDestinyObjectiveProgress {
    self.progress = Some(progress);
    self
  }

  pub fn progress(&self) -> Option<&i32> {
    self.progress.as_ref()
  }

  pub fn reset_progress(&mut self) {
    self.progress = None;
  }

  pub fn set_completion_value(&mut self, completion_value: i32) {
    self.completion_value = Some(completion_value);
  }

  pub fn with_completion_value(mut self, completion_value: i32) -> DestinyQuestsDestinyObjectiveProgress {
    self.completion_value = Some(completion_value);
    self
  }

  pub fn completion_value(&self) -> Option<&i32> {
    self.completion_value.as_ref()
  }

  pub fn reset_completion_value(&mut self) {
    self.completion_value = None;
  }

  pub fn set_complete(&mut self, complete: bool) {
    self.complete = Some(complete);
  }

  pub fn with_complete(mut self, complete: bool) -> DestinyQuestsDestinyObjectiveProgress {
    self.complete = Some(complete);
    self
  }

  pub fn complete(&self) -> Option<&bool> {
    self.complete.as_ref()
  }

  pub fn reset_complete(&mut self) {
    self.complete = None;
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = Some(visible);
  }

  pub fn with_visible(mut self, visible: bool) -> DestinyQuestsDestinyObjectiveProgress {
    self.visible = Some(visible);
    self
  }

  pub fn visible(&self) -> Option<&bool> {
    self.visible.as_ref()
  }

  pub fn reset_visible(&mut self) {
    self.visible = None;
  }

}



