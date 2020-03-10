/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyProgressionStepDefinition : This defines a single Step in a progression (which roughly equates to a level. See DestinyProgressionDefinition for caveats).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyProgressionStepDefinition {
  /// Very rarely, Progressions will have localized text describing the Level of the progression. This will be that localized text, if it exists. Otherwise, the standard appears to be to simply show the level numerically.
  #[serde(rename = "stepName")]
  step_name: Option<String>,
  /// This appears to be, when you \"level up\", whether a visual effect will display and on what entity. See DestinyProgressionStepDisplayEffect for slightly more info.
  #[serde(rename = "displayEffectType")]
  display_effect_type: Option<i32>,
  /// The total amount of progression points/\"experience\" you will need to initially reach this step. If this is the last step and the progression is repeating indefinitely (DestinyProgressionDefinition.repeatLastStep), this will also be the progress needed to level it up further by repeating this step again.
  #[serde(rename = "progressTotal")]
  progress_total: Option<i32>,
  /// A listing of items rewarded as a result of reaching this level.
  #[serde(rename = "rewardItems")]
  reward_items: Option<Vec<::models::DestinyDestinyItemQuantity>>,
  /// If this progression step has a specific icon related to it, this is the icon to show.
  #[serde(rename = "icon")]
  icon: Option<String>
}

impl DestinyDefinitionsDestinyProgressionStepDefinition {
  /// This defines a single Step in a progression (which roughly equates to a level. See DestinyProgressionDefinition for caveats).
  pub fn new() -> DestinyDefinitionsDestinyProgressionStepDefinition {
    DestinyDefinitionsDestinyProgressionStepDefinition {
      step_name: None,
      display_effect_type: None,
      progress_total: None,
      reward_items: None,
      icon: None
    }
  }

  pub fn set_step_name(&mut self, step_name: String) {
    self.step_name = Some(step_name);
  }

  pub fn with_step_name(mut self, step_name: String) -> DestinyDefinitionsDestinyProgressionStepDefinition {
    self.step_name = Some(step_name);
    self
  }

  pub fn step_name(&self) -> Option<&String> {
    self.step_name.as_ref()
  }

  pub fn reset_step_name(&mut self) {
    self.step_name = None;
  }

  pub fn set_display_effect_type(&mut self, display_effect_type: i32) {
    self.display_effect_type = Some(display_effect_type);
  }

  pub fn with_display_effect_type(mut self, display_effect_type: i32) -> DestinyDefinitionsDestinyProgressionStepDefinition {
    self.display_effect_type = Some(display_effect_type);
    self
  }

  pub fn display_effect_type(&self) -> Option<&i32> {
    self.display_effect_type.as_ref()
  }

  pub fn reset_display_effect_type(&mut self) {
    self.display_effect_type = None;
  }

  pub fn set_progress_total(&mut self, progress_total: i32) {
    self.progress_total = Some(progress_total);
  }

  pub fn with_progress_total(mut self, progress_total: i32) -> DestinyDefinitionsDestinyProgressionStepDefinition {
    self.progress_total = Some(progress_total);
    self
  }

  pub fn progress_total(&self) -> Option<&i32> {
    self.progress_total.as_ref()
  }

  pub fn reset_progress_total(&mut self) {
    self.progress_total = None;
  }

  pub fn set_reward_items(&mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) {
    self.reward_items = Some(reward_items);
  }

  pub fn with_reward_items(mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyDefinitionsDestinyProgressionStepDefinition {
    self.reward_items = Some(reward_items);
    self
  }

  pub fn reward_items(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.reward_items.as_ref()
  }

  pub fn reset_reward_items(&mut self) {
    self.reward_items = None;
  }

  pub fn set_icon(&mut self, icon: String) {
    self.icon = Some(icon);
  }

  pub fn with_icon(mut self, icon: String) -> DestinyDefinitionsDestinyProgressionStepDefinition {
    self.icon = Some(icon);
    self
  }

  pub fn icon(&self) -> Option<&String> {
    self.icon.as_ref()
  }

  pub fn reset_icon(&mut self) {
    self.icon = None;
  }

}



