/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemStatBlockDefinition : Information about the item's calculated stats, with as much data as we can find for the stats without having an actual instance of the item.  Note that this means the entire concept of providing these stats is fundamentally insufficient: we cannot predict with 100% accuracy the conditions under which an item can spawn, so we use various heuristics to attempt to simulate the conditions as accurately as possible. Actual stats for items in-game can and will vary, but these should at least be useful base points for comparison and display.  It is also worth noting that some stats, like Magazine size, have further calculations performed on them by scripts in-game and on the game servers that BNet does not have access to. We cannot know how those stats are further transformed, and thus some stats will be inaccurate even on instances of items in BNet vs. how they appear in-game. This is a known limitation of our item statistics, without any planned fix.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemStatBlockDefinition {
  /// If true, the game won't show the \"primary\" stat on this item when you inspect it.  NOTE: This is being manually mapped, because I happen to want it in a block that isn't going to directly create this derivative block.
  #[serde(rename = "disablePrimaryStatDisplay")]
  disable_primary_stat_display: Option<bool>,
  /// If the item's stats are meant to be modified by a DestinyStatGroupDefinition, this will be the identifier for that definition.  If you are using live data or precomputed stats data on the DestinyInventoryItemDefinition.stats.stats property, you don't have to worry about statGroupHash and how it alters stats: the already altered stats are provided to you. But if you want to see how the sausage gets made, or perform computations yourself, this is valuable information.
  #[serde(rename = "statGroupHash")]
  stat_group_hash: Option<i32>,
  /// If you are looking for precomputed values for the stats on a weapon, this is where they are stored. Technically these are the \"Display\" stat values. Please see DestinyStatsDefinition for what Display Stat Values means, it's a very long story... but essentially these are the closest values BNet can get to the item stats that you see in-game.  These stats are keyed by the DestinyStatDefinition's hash identifier for the stat that's found on the item.
  #[serde(rename = "stats")]
  stats: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>>,
  /// A quick and lazy way to determine whether any stat other than the \"primary\" stat is actually visible on the item. Items often have stats that we return in case people find them useful, but they're not part of the \"Stat Group\" and thus we wouldn't display them in our UI. If this is False, then we're not going to display any of these stats other than the primary one.
  #[serde(rename = "hasDisplayableStats")]
  has_displayable_stats: Option<bool>,
  /// This stat is determined to be the \"primary\" stat, and can be looked up in the stats or any other stat collection related to the item.  Use this hash to look up the stat's value using DestinyInventoryItemDefinition.stats.stats, and the renderable data for the primary stat in the related DestinyStatDefinition.
  #[serde(rename = "primaryBaseStatHash")]
  primary_base_stat_hash: Option<i32>
}

impl DestinyDefinitionsDestinyItemStatBlockDefinition {
  /// Information about the item's calculated stats, with as much data as we can find for the stats without having an actual instance of the item.  Note that this means the entire concept of providing these stats is fundamentally insufficient: we cannot predict with 100% accuracy the conditions under which an item can spawn, so we use various heuristics to attempt to simulate the conditions as accurately as possible. Actual stats for items in-game can and will vary, but these should at least be useful base points for comparison and display.  It is also worth noting that some stats, like Magazine size, have further calculations performed on them by scripts in-game and on the game servers that BNet does not have access to. We cannot know how those stats are further transformed, and thus some stats will be inaccurate even on instances of items in BNet vs. how they appear in-game. This is a known limitation of our item statistics, without any planned fix.
  pub fn new() -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    DestinyDefinitionsDestinyItemStatBlockDefinition {
      disable_primary_stat_display: None,
      stat_group_hash: None,
      stats: None,
      has_displayable_stats: None,
      primary_base_stat_hash: None
    }
  }

  pub fn set_disable_primary_stat_display(&mut self, disable_primary_stat_display: bool) {
    self.disable_primary_stat_display = Some(disable_primary_stat_display);
  }

  pub fn with_disable_primary_stat_display(mut self, disable_primary_stat_display: bool) -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    self.disable_primary_stat_display = Some(disable_primary_stat_display);
    self
  }

  pub fn disable_primary_stat_display(&self) -> Option<&bool> {
    self.disable_primary_stat_display.as_ref()
  }

  pub fn reset_disable_primary_stat_display(&mut self) {
    self.disable_primary_stat_display = None;
  }

  pub fn set_stat_group_hash(&mut self, stat_group_hash: i32) {
    self.stat_group_hash = Some(stat_group_hash);
  }

  pub fn with_stat_group_hash(mut self, stat_group_hash: i32) -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    self.stat_group_hash = Some(stat_group_hash);
    self
  }

  pub fn stat_group_hash(&self) -> Option<&i32> {
    self.stat_group_hash.as_ref()
  }

  pub fn reset_stat_group_hash(&mut self) {
    self.stat_group_hash = None;
  }

  pub fn set_stats(&mut self, stats: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>) {
    self.stats = Some(stats);
  }

  pub fn with_stats(mut self, stats: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>) -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    self.stats = Some(stats);
    self
  }

  pub fn stats(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyInventoryItemStatDefinition>> {
    self.stats.as_ref()
  }

  pub fn reset_stats(&mut self) {
    self.stats = None;
  }

  pub fn set_has_displayable_stats(&mut self, has_displayable_stats: bool) {
    self.has_displayable_stats = Some(has_displayable_stats);
  }

  pub fn with_has_displayable_stats(mut self, has_displayable_stats: bool) -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    self.has_displayable_stats = Some(has_displayable_stats);
    self
  }

  pub fn has_displayable_stats(&self) -> Option<&bool> {
    self.has_displayable_stats.as_ref()
  }

  pub fn reset_has_displayable_stats(&mut self) {
    self.has_displayable_stats = None;
  }

  pub fn set_primary_base_stat_hash(&mut self, primary_base_stat_hash: i32) {
    self.primary_base_stat_hash = Some(primary_base_stat_hash);
  }

  pub fn with_primary_base_stat_hash(mut self, primary_base_stat_hash: i32) -> DestinyDefinitionsDestinyItemStatBlockDefinition {
    self.primary_base_stat_hash = Some(primary_base_stat_hash);
    self
  }

  pub fn primary_base_stat_hash(&self) -> Option<&i32> {
    self.primary_base_stat_hash.as_ref()
  }

  pub fn reset_primary_base_stat_hash(&mut self) {
    self.primary_base_stat_hash = None;
  }

}


