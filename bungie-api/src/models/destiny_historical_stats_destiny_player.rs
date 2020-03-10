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
pub struct DestinyHistoricalStatsDestinyPlayer {
  /// Details about the player as they are known in game (platform display name, Destiny emblem)
  #[serde(rename = "destinyUserInfo")]
  destiny_user_info: Option<Value>,
  /// Class of the character if applicable and available.
  #[serde(rename = "characterClass")]
  character_class: Option<String>,
  #[serde(rename = "classHash")]
  class_hash: Option<i32>,
  #[serde(rename = "raceHash")]
  race_hash: Option<i32>,
  #[serde(rename = "genderHash")]
  gender_hash: Option<i32>,
  /// Level of the character if available. Zero if it is not available.
  #[serde(rename = "characterLevel")]
  character_level: Option<i32>,
  /// Light Level of the character if available. Zero if it is not available.
  #[serde(rename = "lightLevel")]
  light_level: Option<i32>,
  /// Details about the player as they are known on BungieNet. This will be undefined if the player has marked their credential private, or does not have a BungieNet account.
  #[serde(rename = "bungieNetUserInfo")]
  bungie_net_user_info: Option<Value>,
  /// Current clan name for the player. This value may be null or an empty string if the user does not have a clan.
  #[serde(rename = "clanName")]
  clan_name: Option<String>,
  /// Current clan tag for the player. This value may be null or an empty string if the user does not have a clan.
  #[serde(rename = "clanTag")]
  clan_tag: Option<String>,
  /// If we know the emblem's hash, this can be used to look up the player's emblem at the time of a match when receiving PGCR data, or otherwise their currently equipped emblem (if we are able to obtain it).
  #[serde(rename = "emblemHash")]
  emblem_hash: Option<i32>
}

impl DestinyHistoricalStatsDestinyPlayer {
  pub fn new() -> DestinyHistoricalStatsDestinyPlayer {
    DestinyHistoricalStatsDestinyPlayer {
      destiny_user_info: None,
      character_class: None,
      class_hash: None,
      race_hash: None,
      gender_hash: None,
      character_level: None,
      light_level: None,
      bungie_net_user_info: None,
      clan_name: None,
      clan_tag: None,
      emblem_hash: None
    }
  }

  pub fn set_destiny_user_info(&mut self, destiny_user_info: Value) {
    self.destiny_user_info = Some(destiny_user_info);
  }

  pub fn with_destiny_user_info(mut self, destiny_user_info: Value) -> DestinyHistoricalStatsDestinyPlayer {
    self.destiny_user_info = Some(destiny_user_info);
    self
  }

  pub fn destiny_user_info(&self) -> Option<&Value> {
    self.destiny_user_info.as_ref()
  }

  pub fn reset_destiny_user_info(&mut self) {
    self.destiny_user_info = None;
  }

  pub fn set_character_class(&mut self, character_class: String) {
    self.character_class = Some(character_class);
  }

  pub fn with_character_class(mut self, character_class: String) -> DestinyHistoricalStatsDestinyPlayer {
    self.character_class = Some(character_class);
    self
  }

  pub fn character_class(&self) -> Option<&String> {
    self.character_class.as_ref()
  }

  pub fn reset_character_class(&mut self) {
    self.character_class = None;
  }

  pub fn set_class_hash(&mut self, class_hash: i32) {
    self.class_hash = Some(class_hash);
  }

  pub fn with_class_hash(mut self, class_hash: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.class_hash = Some(class_hash);
    self
  }

  pub fn class_hash(&self) -> Option<&i32> {
    self.class_hash.as_ref()
  }

  pub fn reset_class_hash(&mut self) {
    self.class_hash = None;
  }

  pub fn set_race_hash(&mut self, race_hash: i32) {
    self.race_hash = Some(race_hash);
  }

  pub fn with_race_hash(mut self, race_hash: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.race_hash = Some(race_hash);
    self
  }

  pub fn race_hash(&self) -> Option<&i32> {
    self.race_hash.as_ref()
  }

  pub fn reset_race_hash(&mut self) {
    self.race_hash = None;
  }

  pub fn set_gender_hash(&mut self, gender_hash: i32) {
    self.gender_hash = Some(gender_hash);
  }

  pub fn with_gender_hash(mut self, gender_hash: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.gender_hash = Some(gender_hash);
    self
  }

  pub fn gender_hash(&self) -> Option<&i32> {
    self.gender_hash.as_ref()
  }

  pub fn reset_gender_hash(&mut self) {
    self.gender_hash = None;
  }

  pub fn set_character_level(&mut self, character_level: i32) {
    self.character_level = Some(character_level);
  }

  pub fn with_character_level(mut self, character_level: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.character_level = Some(character_level);
    self
  }

  pub fn character_level(&self) -> Option<&i32> {
    self.character_level.as_ref()
  }

  pub fn reset_character_level(&mut self) {
    self.character_level = None;
  }

  pub fn set_light_level(&mut self, light_level: i32) {
    self.light_level = Some(light_level);
  }

  pub fn with_light_level(mut self, light_level: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.light_level = Some(light_level);
    self
  }

  pub fn light_level(&self) -> Option<&i32> {
    self.light_level.as_ref()
  }

  pub fn reset_light_level(&mut self) {
    self.light_level = None;
  }

  pub fn set_bungie_net_user_info(&mut self, bungie_net_user_info: Value) {
    self.bungie_net_user_info = Some(bungie_net_user_info);
  }

  pub fn with_bungie_net_user_info(mut self, bungie_net_user_info: Value) -> DestinyHistoricalStatsDestinyPlayer {
    self.bungie_net_user_info = Some(bungie_net_user_info);
    self
  }

  pub fn bungie_net_user_info(&self) -> Option<&Value> {
    self.bungie_net_user_info.as_ref()
  }

  pub fn reset_bungie_net_user_info(&mut self) {
    self.bungie_net_user_info = None;
  }

  pub fn set_clan_name(&mut self, clan_name: String) {
    self.clan_name = Some(clan_name);
  }

  pub fn with_clan_name(mut self, clan_name: String) -> DestinyHistoricalStatsDestinyPlayer {
    self.clan_name = Some(clan_name);
    self
  }

  pub fn clan_name(&self) -> Option<&String> {
    self.clan_name.as_ref()
  }

  pub fn reset_clan_name(&mut self) {
    self.clan_name = None;
  }

  pub fn set_clan_tag(&mut self, clan_tag: String) {
    self.clan_tag = Some(clan_tag);
  }

  pub fn with_clan_tag(mut self, clan_tag: String) -> DestinyHistoricalStatsDestinyPlayer {
    self.clan_tag = Some(clan_tag);
    self
  }

  pub fn clan_tag(&self) -> Option<&String> {
    self.clan_tag.as_ref()
  }

  pub fn reset_clan_tag(&mut self) {
    self.clan_tag = None;
  }

  pub fn set_emblem_hash(&mut self, emblem_hash: i32) {
    self.emblem_hash = Some(emblem_hash);
  }

  pub fn with_emblem_hash(mut self, emblem_hash: i32) -> DestinyHistoricalStatsDestinyPlayer {
    self.emblem_hash = Some(emblem_hash);
    self
  }

  pub fn emblem_hash(&self) -> Option<&i32> {
    self.emblem_hash.as_ref()
  }

  pub fn reset_emblem_hash(&mut self) {
    self.emblem_hash = None;
  }

}


