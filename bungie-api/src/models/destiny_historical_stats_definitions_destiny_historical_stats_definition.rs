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
pub struct DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
  /// Unique programmer friendly ID for this stat
  #[serde(rename = "statId")]
  stat_id: Option<String>,
  /// Statistic group
  #[serde(rename = "group")]
  group: Option<i32>,
  /// Time periods the statistic covers
  #[serde(rename = "periodTypes")]
  period_types: Option<Vec<i32>>,
  /// Game modes where this statistic can be reported.
  #[serde(rename = "modes")]
  modes: Option<Vec<i32>>,
  /// Category for the stat.
  #[serde(rename = "category")]
  category: Option<i32>,
  /// Display name
  #[serde(rename = "statName")]
  stat_name: Option<String>,
  /// Display name abbreviated
  #[serde(rename = "statNameAbbr")]
  stat_name_abbr: Option<String>,
  /// Description of a stat if applicable.
  #[serde(rename = "statDescription")]
  stat_description: Option<String>,
  /// Unit, if any, for the statistic
  #[serde(rename = "unitType")]
  unit_type: Option<i32>,
  /// Optional URI to an icon for the statistic
  #[serde(rename = "iconImage")]
  icon_image: Option<String>,
  /// Optional icon for the statistic
  #[serde(rename = "mergeMethod")]
  merge_method: Option<i32>,
  /// Localized Unit Name for the stat.
  #[serde(rename = "unitLabel")]
  unit_label: Option<String>,
  /// Weight assigned to this stat indicating its relative impressiveness.
  #[serde(rename = "weight")]
  weight: Option<i32>,
  /// The tier associated with this medal - be it implicitly or explicitly.
  #[serde(rename = "medalTierHash")]
  medal_tier_hash: Option<i32>
}

impl DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
  pub fn new() -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
      stat_id: None,
      group: None,
      period_types: None,
      modes: None,
      category: None,
      stat_name: None,
      stat_name_abbr: None,
      stat_description: None,
      unit_type: None,
      icon_image: None,
      merge_method: None,
      unit_label: None,
      weight: None,
      medal_tier_hash: None
    }
  }

  pub fn set_stat_id(&mut self, stat_id: String) {
    self.stat_id = Some(stat_id);
  }

  pub fn with_stat_id(mut self, stat_id: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.stat_id = Some(stat_id);
    self
  }

  pub fn stat_id(&self) -> Option<&String> {
    self.stat_id.as_ref()
  }

  pub fn reset_stat_id(&mut self) {
    self.stat_id = None;
  }

  pub fn set_group(&mut self, group: i32) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&i32> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_period_types(&mut self, period_types: Vec<i32>) {
    self.period_types = Some(period_types);
  }

  pub fn with_period_types(mut self, period_types: Vec<i32>) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.period_types = Some(period_types);
    self
  }

  pub fn period_types(&self) -> Option<&Vec<i32>> {
    self.period_types.as_ref()
  }

  pub fn reset_period_types(&mut self) {
    self.period_types = None;
  }

  pub fn set_modes(&mut self, modes: Vec<i32>) {
    self.modes = Some(modes);
  }

  pub fn with_modes(mut self, modes: Vec<i32>) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.modes = Some(modes);
    self
  }

  pub fn modes(&self) -> Option<&Vec<i32>> {
    self.modes.as_ref()
  }

  pub fn reset_modes(&mut self) {
    self.modes = None;
  }

  pub fn set_category(&mut self, category: i32) {
    self.category = Some(category);
  }

  pub fn with_category(mut self, category: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.category = Some(category);
    self
  }

  pub fn category(&self) -> Option<&i32> {
    self.category.as_ref()
  }

  pub fn reset_category(&mut self) {
    self.category = None;
  }

  pub fn set_stat_name(&mut self, stat_name: String) {
    self.stat_name = Some(stat_name);
  }

  pub fn with_stat_name(mut self, stat_name: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.stat_name = Some(stat_name);
    self
  }

  pub fn stat_name(&self) -> Option<&String> {
    self.stat_name.as_ref()
  }

  pub fn reset_stat_name(&mut self) {
    self.stat_name = None;
  }

  pub fn set_stat_name_abbr(&mut self, stat_name_abbr: String) {
    self.stat_name_abbr = Some(stat_name_abbr);
  }

  pub fn with_stat_name_abbr(mut self, stat_name_abbr: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.stat_name_abbr = Some(stat_name_abbr);
    self
  }

  pub fn stat_name_abbr(&self) -> Option<&String> {
    self.stat_name_abbr.as_ref()
  }

  pub fn reset_stat_name_abbr(&mut self) {
    self.stat_name_abbr = None;
  }

  pub fn set_stat_description(&mut self, stat_description: String) {
    self.stat_description = Some(stat_description);
  }

  pub fn with_stat_description(mut self, stat_description: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.stat_description = Some(stat_description);
    self
  }

  pub fn stat_description(&self) -> Option<&String> {
    self.stat_description.as_ref()
  }

  pub fn reset_stat_description(&mut self) {
    self.stat_description = None;
  }

  pub fn set_unit_type(&mut self, unit_type: i32) {
    self.unit_type = Some(unit_type);
  }

  pub fn with_unit_type(mut self, unit_type: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.unit_type = Some(unit_type);
    self
  }

  pub fn unit_type(&self) -> Option<&i32> {
    self.unit_type.as_ref()
  }

  pub fn reset_unit_type(&mut self) {
    self.unit_type = None;
  }

  pub fn set_icon_image(&mut self, icon_image: String) {
    self.icon_image = Some(icon_image);
  }

  pub fn with_icon_image(mut self, icon_image: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.icon_image = Some(icon_image);
    self
  }

  pub fn icon_image(&self) -> Option<&String> {
    self.icon_image.as_ref()
  }

  pub fn reset_icon_image(&mut self) {
    self.icon_image = None;
  }

  pub fn set_merge_method(&mut self, merge_method: i32) {
    self.merge_method = Some(merge_method);
  }

  pub fn with_merge_method(mut self, merge_method: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.merge_method = Some(merge_method);
    self
  }

  pub fn merge_method(&self) -> Option<&i32> {
    self.merge_method.as_ref()
  }

  pub fn reset_merge_method(&mut self) {
    self.merge_method = None;
  }

  pub fn set_unit_label(&mut self, unit_label: String) {
    self.unit_label = Some(unit_label);
  }

  pub fn with_unit_label(mut self, unit_label: String) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.unit_label = Some(unit_label);
    self
  }

  pub fn unit_label(&self) -> Option<&String> {
    self.unit_label.as_ref()
  }

  pub fn reset_unit_label(&mut self) {
    self.unit_label = None;
  }

  pub fn set_weight(&mut self, weight: i32) {
    self.weight = Some(weight);
  }

  pub fn with_weight(mut self, weight: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.weight = Some(weight);
    self
  }

  pub fn weight(&self) -> Option<&i32> {
    self.weight.as_ref()
  }

  pub fn reset_weight(&mut self) {
    self.weight = None;
  }

  pub fn set_medal_tier_hash(&mut self, medal_tier_hash: i32) {
    self.medal_tier_hash = Some(medal_tier_hash);
  }

  pub fn with_medal_tier_hash(mut self, medal_tier_hash: i32) -> DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition {
    self.medal_tier_hash = Some(medal_tier_hash);
    self
  }

  pub fn medal_tier_hash(&self) -> Option<&i32> {
    self.medal_tier_hash.as_ref()
  }

  pub fn reset_medal_tier_hash(&mut self) {
    self.medal_tier_hash = None;
  }

}



