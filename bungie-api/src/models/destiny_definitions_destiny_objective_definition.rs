/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyObjectiveDefinition : Defines an \"Objective\".  An objective is a specific task you should accomplish in the game. These are referred to by:  - Quest Steps (which are DestinyInventoryItemDefinition entities with Objectives)  - Challenges (which are Objectives defined on an DestinyActivityDefintion)  - Milestones (which refer to Objectives that are defined on both Quest Steps and Activities)  - Anything else that the designers decide to do later.  Objectives have progress, a notion of having been Completed, human readable data describing the task to be accomplished, and a lot of optional tack-on data that can enhance the information provided about the task.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyObjectiveDefinition {
  /// Ideally, this should tell you what your task is. I'm not going to lie to you though. Sometimes this doesn't have useful information at all. Which sucks, but there's nothing either of us can do about it.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  /// The value that the unlock value defined in unlockValueHash must reach in order for the objective to be considered Completed. Used in calculating progress and completion status.
  #[serde(rename = "completionValue")]
  completion_value: Option<i32>,
  /// A shortcut for determining the most restrictive gating that this Objective is set to use. This includes both the dynamic determination of progress and of completion values. See the DestinyGatingScope enum's documentation for more details.
  #[serde(rename = "scope")]
  scope: Option<i32>,
  /// OPTIONAL: a hash identifier for the location at which this objective must be accomplished, if there is a location defined. Look up the DestinyLocationDefinition for this hash for that additional location info.
  #[serde(rename = "locationHash")]
  location_hash: Option<i32>,
  /// If true, the value is allowed to go negative.
  #[serde(rename = "allowNegativeValue")]
  allow_negative_value: Option<bool>,
  /// If true, you can effectively \"un-complete\" this objective if you lose progress after crossing the completion threshold.   If False, once you complete the task it will remain completed forever by locking the value.
  #[serde(rename = "allowValueChangeWhenCompleted")]
  allow_value_change_when_completed: Option<bool>,
  /// If true, completion means having an unlock value less than or equal to the completionValue.  If False, completion means having an unlock value greater than or equal to the completionValue.
  #[serde(rename = "isCountingDownward")]
  is_counting_downward: Option<bool>,
  /// The UI style applied to the objective. It's an enum, take a look at DestinyUnlockValueUIStyle for details of the possible styles. Use this info as you wish to customize your UI.
  #[serde(rename = "valueStyle")]
  value_style: Option<i32>,
  /// Text to describe the progress bar.
  #[serde(rename = "progressDescription")]
  progress_description: Option<String>,
  /// If this objective enables Perks intrinsically, the conditions for that enabling are defined here.
  #[serde(rename = "perks")]
  perks: Option<Value>,
  /// If this objective enables modifications on a player's stats intrinsically, the conditions are defined here.
  #[serde(rename = "stats")]
  stats: Option<Value>,
  /// If nonzero, this is the minimum value at which the objective's progression should be shown. Otherwise, don't show it yet.
  #[serde(rename = "minimumVisibilityThreshold")]
  minimum_visibility_threshold: Option<i32>,
  /// If True, the progress will continue even beyond the point where the objective met its minimum completion requirements. Your UI will have to accommodate it.
  #[serde(rename = "allowOvercompletion")]
  allow_overcompletion: Option<bool>,
  /// If True, you should continue showing the progression value in the UI after it's complete. I mean, we already do that in BNet anyways, but if you want to be better behaved than us you could honor this flag.
  #[serde(rename = "showValueOnComplete")]
  show_value_on_complete: Option<bool>,
  /// The style to use when the objective is completed.
  #[serde(rename = "completedValueStyle")]
  completed_value_style: Option<i32>,
  /// The style to use when the objective is still in progress.
  #[serde(rename = "inProgressValueStyle")]
  in_progress_value_style: Option<i32>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsDestinyObjectiveDefinition {
  /// Defines an \"Objective\".  An objective is a specific task you should accomplish in the game. These are referred to by:  - Quest Steps (which are DestinyInventoryItemDefinition entities with Objectives)  - Challenges (which are Objectives defined on an DestinyActivityDefintion)  - Milestones (which refer to Objectives that are defined on both Quest Steps and Activities)  - Anything else that the designers decide to do later.  Objectives have progress, a notion of having been Completed, human readable data describing the task to be accomplished, and a lot of optional tack-on data that can enhance the information provided about the task.
  pub fn new() -> DestinyDefinitionsDestinyObjectiveDefinition {
    DestinyDefinitionsDestinyObjectiveDefinition {
      display_properties: None,
      completion_value: None,
      scope: None,
      location_hash: None,
      allow_negative_value: None,
      allow_value_change_when_completed: None,
      is_counting_downward: None,
      value_style: None,
      progress_description: None,
      perks: None,
      stats: None,
      minimum_visibility_threshold: None,
      allow_overcompletion: None,
      show_value_on_complete: None,
      completed_value_style: None,
      in_progress_value_style: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_completion_value(&mut self, completion_value: i32) {
    self.completion_value = Some(completion_value);
  }

  pub fn with_completion_value(mut self, completion_value: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.completion_value = Some(completion_value);
    self
  }

  pub fn completion_value(&self) -> Option<&i32> {
    self.completion_value.as_ref()
  }

  pub fn reset_completion_value(&mut self) {
    self.completion_value = None;
  }

  pub fn set_scope(&mut self, scope: i32) {
    self.scope = Some(scope);
  }

  pub fn with_scope(mut self, scope: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.scope = Some(scope);
    self
  }

  pub fn scope(&self) -> Option<&i32> {
    self.scope.as_ref()
  }

  pub fn reset_scope(&mut self) {
    self.scope = None;
  }

  pub fn set_location_hash(&mut self, location_hash: i32) {
    self.location_hash = Some(location_hash);
  }

  pub fn with_location_hash(mut self, location_hash: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.location_hash = Some(location_hash);
    self
  }

  pub fn location_hash(&self) -> Option<&i32> {
    self.location_hash.as_ref()
  }

  pub fn reset_location_hash(&mut self) {
    self.location_hash = None;
  }

  pub fn set_allow_negative_value(&mut self, allow_negative_value: bool) {
    self.allow_negative_value = Some(allow_negative_value);
  }

  pub fn with_allow_negative_value(mut self, allow_negative_value: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.allow_negative_value = Some(allow_negative_value);
    self
  }

  pub fn allow_negative_value(&self) -> Option<&bool> {
    self.allow_negative_value.as_ref()
  }

  pub fn reset_allow_negative_value(&mut self) {
    self.allow_negative_value = None;
  }

  pub fn set_allow_value_change_when_completed(&mut self, allow_value_change_when_completed: bool) {
    self.allow_value_change_when_completed = Some(allow_value_change_when_completed);
  }

  pub fn with_allow_value_change_when_completed(mut self, allow_value_change_when_completed: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.allow_value_change_when_completed = Some(allow_value_change_when_completed);
    self
  }

  pub fn allow_value_change_when_completed(&self) -> Option<&bool> {
    self.allow_value_change_when_completed.as_ref()
  }

  pub fn reset_allow_value_change_when_completed(&mut self) {
    self.allow_value_change_when_completed = None;
  }

  pub fn set_is_counting_downward(&mut self, is_counting_downward: bool) {
    self.is_counting_downward = Some(is_counting_downward);
  }

  pub fn with_is_counting_downward(mut self, is_counting_downward: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.is_counting_downward = Some(is_counting_downward);
    self
  }

  pub fn is_counting_downward(&self) -> Option<&bool> {
    self.is_counting_downward.as_ref()
  }

  pub fn reset_is_counting_downward(&mut self) {
    self.is_counting_downward = None;
  }

  pub fn set_value_style(&mut self, value_style: i32) {
    self.value_style = Some(value_style);
  }

  pub fn with_value_style(mut self, value_style: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.value_style = Some(value_style);
    self
  }

  pub fn value_style(&self) -> Option<&i32> {
    self.value_style.as_ref()
  }

  pub fn reset_value_style(&mut self) {
    self.value_style = None;
  }

  pub fn set_progress_description(&mut self, progress_description: String) {
    self.progress_description = Some(progress_description);
  }

  pub fn with_progress_description(mut self, progress_description: String) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.progress_description = Some(progress_description);
    self
  }

  pub fn progress_description(&self) -> Option<&String> {
    self.progress_description.as_ref()
  }

  pub fn reset_progress_description(&mut self) {
    self.progress_description = None;
  }

  pub fn set_perks(&mut self, perks: Value) {
    self.perks = Some(perks);
  }

  pub fn with_perks(mut self, perks: Value) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.perks = Some(perks);
    self
  }

  pub fn perks(&self) -> Option<&Value> {
    self.perks.as_ref()
  }

  pub fn reset_perks(&mut self) {
    self.perks = None;
  }

  pub fn set_stats(&mut self, stats: Value) {
    self.stats = Some(stats);
  }

  pub fn with_stats(mut self, stats: Value) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.stats = Some(stats);
    self
  }

  pub fn stats(&self) -> Option<&Value> {
    self.stats.as_ref()
  }

  pub fn reset_stats(&mut self) {
    self.stats = None;
  }

  pub fn set_minimum_visibility_threshold(&mut self, minimum_visibility_threshold: i32) {
    self.minimum_visibility_threshold = Some(minimum_visibility_threshold);
  }

  pub fn with_minimum_visibility_threshold(mut self, minimum_visibility_threshold: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.minimum_visibility_threshold = Some(minimum_visibility_threshold);
    self
  }

  pub fn minimum_visibility_threshold(&self) -> Option<&i32> {
    self.minimum_visibility_threshold.as_ref()
  }

  pub fn reset_minimum_visibility_threshold(&mut self) {
    self.minimum_visibility_threshold = None;
  }

  pub fn set_allow_overcompletion(&mut self, allow_overcompletion: bool) {
    self.allow_overcompletion = Some(allow_overcompletion);
  }

  pub fn with_allow_overcompletion(mut self, allow_overcompletion: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.allow_overcompletion = Some(allow_overcompletion);
    self
  }

  pub fn allow_overcompletion(&self) -> Option<&bool> {
    self.allow_overcompletion.as_ref()
  }

  pub fn reset_allow_overcompletion(&mut self) {
    self.allow_overcompletion = None;
  }

  pub fn set_show_value_on_complete(&mut self, show_value_on_complete: bool) {
    self.show_value_on_complete = Some(show_value_on_complete);
  }

  pub fn with_show_value_on_complete(mut self, show_value_on_complete: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.show_value_on_complete = Some(show_value_on_complete);
    self
  }

  pub fn show_value_on_complete(&self) -> Option<&bool> {
    self.show_value_on_complete.as_ref()
  }

  pub fn reset_show_value_on_complete(&mut self) {
    self.show_value_on_complete = None;
  }

  pub fn set_completed_value_style(&mut self, completed_value_style: i32) {
    self.completed_value_style = Some(completed_value_style);
  }

  pub fn with_completed_value_style(mut self, completed_value_style: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.completed_value_style = Some(completed_value_style);
    self
  }

  pub fn completed_value_style(&self) -> Option<&i32> {
    self.completed_value_style.as_ref()
  }

  pub fn reset_completed_value_style(&mut self) {
    self.completed_value_style = None;
  }

  pub fn set_in_progress_value_style(&mut self, in_progress_value_style: i32) {
    self.in_progress_value_style = Some(in_progress_value_style);
  }

  pub fn with_in_progress_value_style(mut self, in_progress_value_style: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.in_progress_value_style = Some(in_progress_value_style);
    self
  }

  pub fn in_progress_value_style(&self) -> Option<&i32> {
    self.in_progress_value_style.as_ref()
  }

  pub fn reset_in_progress_value_style(&mut self) {
    self.in_progress_value_style = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsDestinyObjectiveDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}



