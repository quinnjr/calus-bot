/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemPerkEntryDefinition : An intrinsic perk on an item, and the requirements for it to be activated.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemPerkEntryDefinition {
  /// If this perk is not active, this is the string to show for why it's not providing its benefits.
  #[serde(rename = "requirementDisplayString")]
  requirement_display_string: Option<String>,
  /// A hash identifier for the DestinySandboxPerkDefinition being provided on the item.
  #[serde(rename = "perkHash")]
  perk_hash: Option<i32>,
  /// Indicates whether this perk should be shown, or if it should be shown disabled.
  #[serde(rename = "perkVisibility")]
  perk_visibility: Option<i32>
}

impl DestinyDefinitionsDestinyItemPerkEntryDefinition {
  /// An intrinsic perk on an item, and the requirements for it to be activated.
  pub fn new() -> DestinyDefinitionsDestinyItemPerkEntryDefinition {
    DestinyDefinitionsDestinyItemPerkEntryDefinition {
      requirement_display_string: None,
      perk_hash: None,
      perk_visibility: None
    }
  }

  pub fn set_requirement_display_string(&mut self, requirement_display_string: String) {
    self.requirement_display_string = Some(requirement_display_string);
  }

  pub fn with_requirement_display_string(mut self, requirement_display_string: String) -> DestinyDefinitionsDestinyItemPerkEntryDefinition {
    self.requirement_display_string = Some(requirement_display_string);
    self
  }

  pub fn requirement_display_string(&self) -> Option<&String> {
    self.requirement_display_string.as_ref()
  }

  pub fn reset_requirement_display_string(&mut self) {
    self.requirement_display_string = None;
  }

  pub fn set_perk_hash(&mut self, perk_hash: i32) {
    self.perk_hash = Some(perk_hash);
  }

  pub fn with_perk_hash(mut self, perk_hash: i32) -> DestinyDefinitionsDestinyItemPerkEntryDefinition {
    self.perk_hash = Some(perk_hash);
    self
  }

  pub fn perk_hash(&self) -> Option<&i32> {
    self.perk_hash.as_ref()
  }

  pub fn reset_perk_hash(&mut self) {
    self.perk_hash = None;
  }

  pub fn set_perk_visibility(&mut self, perk_visibility: i32) {
    self.perk_visibility = Some(perk_visibility);
  }

  pub fn with_perk_visibility(mut self, perk_visibility: i32) -> DestinyDefinitionsDestinyItemPerkEntryDefinition {
    self.perk_visibility = Some(perk_visibility);
    self
  }

  pub fn perk_visibility(&self) -> Option<&i32> {
    self.perk_visibility.as_ref()
  }

  pub fn reset_perk_visibility(&mut self) {
    self.perk_visibility = None;
  }

}


