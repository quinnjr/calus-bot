/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsSourcesDestinyItemSourceDefinition : Properties of a DestinyInventoryItemDefinition that store all of the information we were able to discern about how the item spawns, and where you can find the item.  Items will have many of these sources, one per level at which it spawns, to try and give more granular data about where items spawn for specific level ranges.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsSourcesDestinyItemSourceDefinition {
  /// The level at which the item spawns. Essentially the Primary Key for this source data: there will be multiple of these source entries per item that has source data, grouped by the level at which the item spawns.
  #[serde(rename = "level")]
  level: Option<i32>,
  /// The minimum Quality at which the item spawns for this level. Examine DestinyInventoryItemDefinition for more information about what Quality means. Just don't ask Phaedrus about it, he'll never stop talking and you'll have to write a book about it.
  #[serde(rename = "minQuality")]
  min_quality: Option<i32>,
  /// The maximum quality at which the item spawns for this level.
  #[serde(rename = "maxQuality")]
  max_quality: Option<i32>,
  /// The minimum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing.
  #[serde(rename = "minLevelRequired")]
  min_level_required: Option<i32>,
  /// The maximum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing.
  #[serde(rename = "maxLevelRequired")]
  max_level_required: Option<i32>,
  /// The stats computed for this level/quality range.
  #[serde(rename = "computedStats")]
  computed_stats: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>>,
  /// The DestinyRewardSourceDefinitions found that can spawn the item at this level.
  #[serde(rename = "sourceHashes")]
  source_hashes: Option<Vec<i32>>
}

impl DestinyDefinitionsSourcesDestinyItemSourceDefinition {
  /// Properties of a DestinyInventoryItemDefinition that store all of the information we were able to discern about how the item spawns, and where you can find the item.  Items will have many of these sources, one per level at which it spawns, to try and give more granular data about where items spawn for specific level ranges.
  pub fn new() -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    DestinyDefinitionsSourcesDestinyItemSourceDefinition {
      level: None,
      min_quality: None,
      max_quality: None,
      min_level_required: None,
      max_level_required: None,
      computed_stats: None,
      source_hashes: None
    }
  }

  pub fn set_level(&mut self, level: i32) {
    self.level = Some(level);
  }

  pub fn with_level(mut self, level: i32) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.level = Some(level);
    self
  }

  pub fn level(&self) -> Option<&i32> {
    self.level.as_ref()
  }

  pub fn reset_level(&mut self) {
    self.level = None;
  }

  pub fn set_min_quality(&mut self, min_quality: i32) {
    self.min_quality = Some(min_quality);
  }

  pub fn with_min_quality(mut self, min_quality: i32) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.min_quality = Some(min_quality);
    self
  }

  pub fn min_quality(&self) -> Option<&i32> {
    self.min_quality.as_ref()
  }

  pub fn reset_min_quality(&mut self) {
    self.min_quality = None;
  }

  pub fn set_max_quality(&mut self, max_quality: i32) {
    self.max_quality = Some(max_quality);
  }

  pub fn with_max_quality(mut self, max_quality: i32) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.max_quality = Some(max_quality);
    self
  }

  pub fn max_quality(&self) -> Option<&i32> {
    self.max_quality.as_ref()
  }

  pub fn reset_max_quality(&mut self) {
    self.max_quality = None;
  }

  pub fn set_min_level_required(&mut self, min_level_required: i32) {
    self.min_level_required = Some(min_level_required);
  }

  pub fn with_min_level_required(mut self, min_level_required: i32) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.min_level_required = Some(min_level_required);
    self
  }

  pub fn min_level_required(&self) -> Option<&i32> {
    self.min_level_required.as_ref()
  }

  pub fn reset_min_level_required(&mut self) {
    self.min_level_required = None;
  }

  pub fn set_max_level_required(&mut self, max_level_required: i32) {
    self.max_level_required = Some(max_level_required);
  }

  pub fn with_max_level_required(mut self, max_level_required: i32) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.max_level_required = Some(max_level_required);
    self
  }

  pub fn max_level_required(&self) -> Option<&i32> {
    self.max_level_required.as_ref()
  }

  pub fn reset_max_level_required(&mut self) {
    self.max_level_required = None;
  }

  pub fn set_computed_stats(&mut self, computed_stats: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>) {
    self.computed_stats = Some(computed_stats);
  }

  pub fn with_computed_stats(mut self, computed_stats: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.computed_stats = Some(computed_stats);
    self
  }

  pub fn computed_stats(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>> {
    self.computed_stats.as_ref()
  }

  pub fn reset_computed_stats(&mut self) {
    self.computed_stats = None;
  }

  pub fn set_source_hashes(&mut self, source_hashes: Vec<i32>) {
    self.source_hashes = Some(source_hashes);
  }

  pub fn with_source_hashes(mut self, source_hashes: Vec<i32>) -> DestinyDefinitionsSourcesDestinyItemSourceDefinition {
    self.source_hashes = Some(source_hashes);
    self
  }

  pub fn source_hashes(&self) -> Option<&Vec<i32>> {
    self.source_hashes.as_ref()
  }

  pub fn reset_source_hashes(&mut self) {
    self.source_hashes = None;
  }

}



