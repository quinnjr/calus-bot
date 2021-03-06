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
pub struct DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
  /// An identifier, unique within the Artifact, for this specific tier.
  #[serde(rename = "tierHash")]
  tier_hash: Option<i32>,
  /// The human readable title of this tier, if any.
  #[serde(rename = "displayTitle")]
  display_title: Option<String>,
  /// A string representing the localized minimum requirement text for this Tier, if any.
  #[serde(rename = "progressRequirementMessage")]
  progress_requirement_message: Option<String>,
  /// The items that can be earned within this tier.
  #[serde(rename = "items")]
  items: Option<Vec<::models::DestinyDefinitionsArtifactsDestinyArtifactTierItemDefinition>>,
  /// The minimum number of \"unlock points\" that you must have used before you can unlock items from this tier.
  #[serde(rename = "minimumUnlockPointsUsedRequirement")]
  minimum_unlock_points_used_requirement: Option<i32>
}

impl DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
  pub fn new() -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
      tier_hash: None,
      display_title: None,
      progress_requirement_message: None,
      items: None,
      minimum_unlock_points_used_requirement: None
    }
  }

  pub fn set_tier_hash(&mut self, tier_hash: i32) {
    self.tier_hash = Some(tier_hash);
  }

  pub fn with_tier_hash(mut self, tier_hash: i32) -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    self.tier_hash = Some(tier_hash);
    self
  }

  pub fn tier_hash(&self) -> Option<&i32> {
    self.tier_hash.as_ref()
  }

  pub fn reset_tier_hash(&mut self) {
    self.tier_hash = None;
  }

  pub fn set_display_title(&mut self, display_title: String) {
    self.display_title = Some(display_title);
  }

  pub fn with_display_title(mut self, display_title: String) -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    self.display_title = Some(display_title);
    self
  }

  pub fn display_title(&self) -> Option<&String> {
    self.display_title.as_ref()
  }

  pub fn reset_display_title(&mut self) {
    self.display_title = None;
  }

  pub fn set_progress_requirement_message(&mut self, progress_requirement_message: String) {
    self.progress_requirement_message = Some(progress_requirement_message);
  }

  pub fn with_progress_requirement_message(mut self, progress_requirement_message: String) -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    self.progress_requirement_message = Some(progress_requirement_message);
    self
  }

  pub fn progress_requirement_message(&self) -> Option<&String> {
    self.progress_requirement_message.as_ref()
  }

  pub fn reset_progress_requirement_message(&mut self) {
    self.progress_requirement_message = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::DestinyDefinitionsArtifactsDestinyArtifactTierItemDefinition>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::DestinyDefinitionsArtifactsDestinyArtifactTierItemDefinition>) -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::DestinyDefinitionsArtifactsDestinyArtifactTierItemDefinition>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_minimum_unlock_points_used_requirement(&mut self, minimum_unlock_points_used_requirement: i32) {
    self.minimum_unlock_points_used_requirement = Some(minimum_unlock_points_used_requirement);
  }

  pub fn with_minimum_unlock_points_used_requirement(mut self, minimum_unlock_points_used_requirement: i32) -> DestinyDefinitionsArtifactsDestinyArtifactTierDefinition {
    self.minimum_unlock_points_used_requirement = Some(minimum_unlock_points_used_requirement);
    self
  }

  pub fn minimum_unlock_points_used_requirement(&self) -> Option<&i32> {
    self.minimum_unlock_points_used_requirement.as_ref()
  }

  pub fn reset_minimum_unlock_points_used_requirement(&mut self) {
    self.minimum_unlock_points_used_requirement = None;
  }

}



