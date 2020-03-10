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
pub struct DestinyHistoricalStatsDestinyLeaderboardEntry {
  /// Where this player ranks on the leaderboard. A value of 1 is the top rank.
  #[serde(rename = "rank")]
  rank: Option<i32>,
  /// Identity details of the player
  #[serde(rename = "player")]
  player: Option<Value>,
  /// ID of the player's best character for the reported stat.
  #[serde(rename = "characterId")]
  character_id: Option<i64>,
  /// Value of the stat for this player
  #[serde(rename = "value")]
  value: Option<Value>
}

impl DestinyHistoricalStatsDestinyLeaderboardEntry {
  pub fn new() -> DestinyHistoricalStatsDestinyLeaderboardEntry {
    DestinyHistoricalStatsDestinyLeaderboardEntry {
      rank: None,
      player: None,
      character_id: None,
      value: None
    }
  }

  pub fn set_rank(&mut self, rank: i32) {
    self.rank = Some(rank);
  }

  pub fn with_rank(mut self, rank: i32) -> DestinyHistoricalStatsDestinyLeaderboardEntry {
    self.rank = Some(rank);
    self
  }

  pub fn rank(&self) -> Option<&i32> {
    self.rank.as_ref()
  }

  pub fn reset_rank(&mut self) {
    self.rank = None;
  }

  pub fn set_player(&mut self, player: Value) {
    self.player = Some(player);
  }

  pub fn with_player(mut self, player: Value) -> DestinyHistoricalStatsDestinyLeaderboardEntry {
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

  pub fn with_character_id(mut self, character_id: i64) -> DestinyHistoricalStatsDestinyLeaderboardEntry {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i64> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

  pub fn set_value(&mut self, value: Value) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: Value) -> DestinyHistoricalStatsDestinyLeaderboardEntry {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&Value> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}


