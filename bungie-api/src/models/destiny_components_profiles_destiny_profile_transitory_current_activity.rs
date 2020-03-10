/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity : If you are playing in an activity, this is some information about it.  Note that we cannot guarantee any of this resembles what ends up in the PGCR in any way. They are sourced by two entirely separate systems with their own logic, and the one we source this data from should be considered non-authoritative in comparison.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
  /// When the activity started.
  #[serde(rename = "startTime")]
  start_time: Option<String>,
  /// If you're still in it but it \"ended\" (like when folks are dancing around the loot after they beat a boss), this is when the activity ended.
  #[serde(rename = "endTime")]
  end_time: Option<String>,
  /// This is what our non-authoritative source thought the score was.
  #[serde(rename = "score")]
  score: Option<f32>,
  /// If you have human opponents, this is the highest opposing team's score.
  #[serde(rename = "highestOpposingFactionScore")]
  highest_opposing_faction_score: Option<f32>,
  /// This is how many human or poorly crafted aimbot opponents you have.
  #[serde(rename = "numberOfOpponents")]
  number_of_opponents: Option<i32>,
  /// This is how many human or poorly crafted aimbots are on your team.
  #[serde(rename = "numberOfPlayers")]
  number_of_players: Option<i32>
}

impl DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
  /// If you are playing in an activity, this is some information about it.  Note that we cannot guarantee any of this resembles what ends up in the PGCR in any way. They are sourced by two entirely separate systems with their own logic, and the one we source this data from should be considered non-authoritative in comparison.
  pub fn new() -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
      start_time: None,
      end_time: None,
      score: None,
      highest_opposing_faction_score: None,
      number_of_opponents: None,
      number_of_players: None
    }
  }

  pub fn set_start_time(&mut self, start_time: String) {
    self.start_time = Some(start_time);
  }

  pub fn with_start_time(mut self, start_time: String) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.start_time = Some(start_time);
    self
  }

  pub fn start_time(&self) -> Option<&String> {
    self.start_time.as_ref()
  }

  pub fn reset_start_time(&mut self) {
    self.start_time = None;
  }

  pub fn set_end_time(&mut self, end_time: String) {
    self.end_time = Some(end_time);
  }

  pub fn with_end_time(mut self, end_time: String) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.end_time = Some(end_time);
    self
  }

  pub fn end_time(&self) -> Option<&String> {
    self.end_time.as_ref()
  }

  pub fn reset_end_time(&mut self) {
    self.end_time = None;
  }

  pub fn set_score(&mut self, score: f32) {
    self.score = Some(score);
  }

  pub fn with_score(mut self, score: f32) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.score = Some(score);
    self
  }

  pub fn score(&self) -> Option<&f32> {
    self.score.as_ref()
  }

  pub fn reset_score(&mut self) {
    self.score = None;
  }

  pub fn set_highest_opposing_faction_score(&mut self, highest_opposing_faction_score: f32) {
    self.highest_opposing_faction_score = Some(highest_opposing_faction_score);
  }

  pub fn with_highest_opposing_faction_score(mut self, highest_opposing_faction_score: f32) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.highest_opposing_faction_score = Some(highest_opposing_faction_score);
    self
  }

  pub fn highest_opposing_faction_score(&self) -> Option<&f32> {
    self.highest_opposing_faction_score.as_ref()
  }

  pub fn reset_highest_opposing_faction_score(&mut self) {
    self.highest_opposing_faction_score = None;
  }

  pub fn set_number_of_opponents(&mut self, number_of_opponents: i32) {
    self.number_of_opponents = Some(number_of_opponents);
  }

  pub fn with_number_of_opponents(mut self, number_of_opponents: i32) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.number_of_opponents = Some(number_of_opponents);
    self
  }

  pub fn number_of_opponents(&self) -> Option<&i32> {
    self.number_of_opponents.as_ref()
  }

  pub fn reset_number_of_opponents(&mut self) {
    self.number_of_opponents = None;
  }

  pub fn set_number_of_players(&mut self, number_of_players: i32) {
    self.number_of_players = Some(number_of_players);
  }

  pub fn with_number_of_players(mut self, number_of_players: i32) -> DestinyComponentsProfilesDestinyProfileTransitoryCurrentActivity {
    self.number_of_players = Some(number_of_players);
    self
  }

  pub fn number_of_players(&self) -> Option<&i32> {
    self.number_of_players.as_ref()
  }

  pub fn reset_number_of_players(&mut self) {
    self.number_of_players = None;
  }

}


