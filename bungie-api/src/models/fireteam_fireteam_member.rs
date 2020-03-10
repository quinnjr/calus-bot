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
pub struct FireteamFireteamMember {
  #[serde(rename = "destinyUserInfo")]
  destiny_user_info: Option<::models::FireteamFireteamUserInfoCard>,
  #[serde(rename = "bungieNetUserInfo")]
  bungie_net_user_info: Option<::models::UserUserInfoCard>,
  #[serde(rename = "characterId")]
  character_id: Option<i64>,
  #[serde(rename = "dateJoined")]
  date_joined: Option<String>,
  #[serde(rename = "hasMicrophone")]
  has_microphone: Option<bool>,
  #[serde(rename = "lastPlatformInviteAttemptDate")]
  last_platform_invite_attempt_date: Option<String>,
  #[serde(rename = "lastPlatformInviteAttemptResult")]
  last_platform_invite_attempt_result: Option<i32>
}

impl FireteamFireteamMember {
  pub fn new() -> FireteamFireteamMember {
    FireteamFireteamMember {
      destiny_user_info: None,
      bungie_net_user_info: None,
      character_id: None,
      date_joined: None,
      has_microphone: None,
      last_platform_invite_attempt_date: None,
      last_platform_invite_attempt_result: None
    }
  }

  pub fn set_destiny_user_info(&mut self, destiny_user_info: ::models::FireteamFireteamUserInfoCard) {
    self.destiny_user_info = Some(destiny_user_info);
  }

  pub fn with_destiny_user_info(mut self, destiny_user_info: ::models::FireteamFireteamUserInfoCard) -> FireteamFireteamMember {
    self.destiny_user_info = Some(destiny_user_info);
    self
  }

  pub fn destiny_user_info(&self) -> Option<&::models::FireteamFireteamUserInfoCard> {
    self.destiny_user_info.as_ref()
  }

  pub fn reset_destiny_user_info(&mut self) {
    self.destiny_user_info = None;
  }

  pub fn set_bungie_net_user_info(&mut self, bungie_net_user_info: ::models::UserUserInfoCard) {
    self.bungie_net_user_info = Some(bungie_net_user_info);
  }

  pub fn with_bungie_net_user_info(mut self, bungie_net_user_info: ::models::UserUserInfoCard) -> FireteamFireteamMember {
    self.bungie_net_user_info = Some(bungie_net_user_info);
    self
  }

  pub fn bungie_net_user_info(&self) -> Option<&::models::UserUserInfoCard> {
    self.bungie_net_user_info.as_ref()
  }

  pub fn reset_bungie_net_user_info(&mut self) {
    self.bungie_net_user_info = None;
  }

  pub fn set_character_id(&mut self, character_id: i64) {
    self.character_id = Some(character_id);
  }

  pub fn with_character_id(mut self, character_id: i64) -> FireteamFireteamMember {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i64> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

  pub fn set_date_joined(&mut self, date_joined: String) {
    self.date_joined = Some(date_joined);
  }

  pub fn with_date_joined(mut self, date_joined: String) -> FireteamFireteamMember {
    self.date_joined = Some(date_joined);
    self
  }

  pub fn date_joined(&self) -> Option<&String> {
    self.date_joined.as_ref()
  }

  pub fn reset_date_joined(&mut self) {
    self.date_joined = None;
  }

  pub fn set_has_microphone(&mut self, has_microphone: bool) {
    self.has_microphone = Some(has_microphone);
  }

  pub fn with_has_microphone(mut self, has_microphone: bool) -> FireteamFireteamMember {
    self.has_microphone = Some(has_microphone);
    self
  }

  pub fn has_microphone(&self) -> Option<&bool> {
    self.has_microphone.as_ref()
  }

  pub fn reset_has_microphone(&mut self) {
    self.has_microphone = None;
  }

  pub fn set_last_platform_invite_attempt_date(&mut self, last_platform_invite_attempt_date: String) {
    self.last_platform_invite_attempt_date = Some(last_platform_invite_attempt_date);
  }

  pub fn with_last_platform_invite_attempt_date(mut self, last_platform_invite_attempt_date: String) -> FireteamFireteamMember {
    self.last_platform_invite_attempt_date = Some(last_platform_invite_attempt_date);
    self
  }

  pub fn last_platform_invite_attempt_date(&self) -> Option<&String> {
    self.last_platform_invite_attempt_date.as_ref()
  }

  pub fn reset_last_platform_invite_attempt_date(&mut self) {
    self.last_platform_invite_attempt_date = None;
  }

  pub fn set_last_platform_invite_attempt_result(&mut self, last_platform_invite_attempt_result: i32) {
    self.last_platform_invite_attempt_result = Some(last_platform_invite_attempt_result);
  }

  pub fn with_last_platform_invite_attempt_result(mut self, last_platform_invite_attempt_result: i32) -> FireteamFireteamMember {
    self.last_platform_invite_attempt_result = Some(last_platform_invite_attempt_result);
    self
  }

  pub fn last_platform_invite_attempt_result(&self) -> Option<&i32> {
    self.last_platform_invite_attempt_result.as_ref()
  }

  pub fn reset_last_platform_invite_attempt_result(&mut self) {
    self.last_platform_invite_attempt_result = None;
  }

}



