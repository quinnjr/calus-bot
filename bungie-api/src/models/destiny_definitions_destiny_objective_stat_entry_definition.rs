/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyObjectiveStatEntryDefinition : Defines the conditions under which stat modifications will be applied to a Character while participating in an objective.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
  /// The stat being modified, and the value used.
  #[serde(rename = "stat")]
  stat: Option<Value>,
  /// Whether it will be applied as long as the objective is active, when it's completed, or until it's completed.
  #[serde(rename = "style")]
  style: Option<i32>
}

impl DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
  /// Defines the conditions under which stat modifications will be applied to a Character while participating in an objective.
  pub fn new() -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
      stat: None,
      style: None
    }
  }

  pub fn set_stat(&mut self, stat: Value) {
    self.stat = Some(stat);
  }

  pub fn with_stat(mut self, stat: Value) -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    self.stat = Some(stat);
    self
  }

  pub fn stat(&self) -> Option<&Value> {
    self.stat.as_ref()
  }

  pub fn reset_stat(&mut self) {
    self.stat = None;
  }

  pub fn set_style(&mut self, style: i32) {
    self.style = Some(style);
  }

  pub fn with_style(mut self, style: i32) -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    self.style = Some(style);
    self
  }

  pub fn style(&self) -> Option<&i32> {
    self.style.as_ref()
  }

  pub fn reset_style(&mut self) {
    self.style = None;
  }

}



