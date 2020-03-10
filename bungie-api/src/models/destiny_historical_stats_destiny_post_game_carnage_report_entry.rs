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
pub struct DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
  /// Standing of the player
  #[serde(rename = "standing")]
  standing: Option<i32>,
  /// Score of the player if available
  #[serde(rename = "score")]
  score: Option<Value>,
  /// Identity details of the player
  #[serde(rename = "player")]
  player: Option<Value>,
  /// ID of the player's character used in the activity.
  #[serde(rename = "characterId")]
  character_id: Option<i64>,
  /// Collection of stats for the player in this activity.
  #[serde(rename = "values")]
  values: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>>,
  /// Extended data extracted from the activity blob.
  #[serde(rename = "extended")]
  extended: Option<Value>
}

impl DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
  pub fn new() -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
      standing: None,
      score: None,
      player: None,
      character_id: None,
      values: None,
      extended: None
    }
  }

  pub fn set_standing(&mut self, standing: i32) {
    self.standing = Some(standing);
  }

  pub fn with_standing(mut self, standing: i32) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.standing = Some(standing);
    self
  }

  pub fn standing(&self) -> Option<&i32> {
    self.standing.as_ref()
  }

  pub fn reset_standing(&mut self) {
    self.standing = None;
  }

  pub fn set_score(&mut self, score: Value) {
    self.score = Some(score);
  }

  pub fn with_score(mut self, score: Value) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.score = Some(score);
    self
  }

  pub fn score(&self) -> Option<&Value> {
    self.score.as_ref()
  }

  pub fn reset_score(&mut self) {
    self.score = None;
  }

  pub fn set_player(&mut self, player: Value) {
    self.player = Some(player);
  }

  pub fn with_player(mut self, player: Value) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.player = Some(player);
    self
  }

  pub fn player(&self) -> Option<&Value> {
    self.player.as_ref()
  }

  pub fn reset_player(&mut self) {
    self.player = None;
  }

  pub fn set_character_id(&mut self, character_id: i64) {
    self.character_id = Some(character_id);
  }

  pub fn with_character_id(mut self, character_id: i64) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i64> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

  pub fn set_values(&mut self, values: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) {
    self.values = Some(values);
  }

  pub fn with_values(mut self, values: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.values = Some(values);
    self
  }

  pub fn values(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>> {
    self.values.as_ref()
  }

  pub fn reset_values(&mut self) {
    self.values = None;
  }

  pub fn set_extended(&mut self, extended: Value) {
    self.extended = Some(extended);
  }

  pub fn with_extended(mut self, extended: Value) -> DestinyHistoricalStatsDestinyPostGameCarnageReportEntry {
    self.extended = Some(extended);
    self
  }

  pub fn extended(&self) -> Option<&Value> {
    self.extended.as_ref()
  }

  pub fn reset_extended(&mut self) {
    self.extended = None;
  }

}


