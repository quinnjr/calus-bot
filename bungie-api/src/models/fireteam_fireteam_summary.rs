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
pub struct FireteamFireteamSummary {
  #[serde(rename = "fireteamId")]
  fireteam_id: Option<i64>,
  #[serde(rename = "groupId")]
  group_id: Option<i64>,
  #[serde(rename = "platform")]
  platform: Option<i32>,
  #[serde(rename = "activityType")]
  activity_type: Option<i32>,
  #[serde(rename = "isImmediate")]
  is_immediate: Option<bool>,
  #[serde(rename = "scheduledTime")]
  scheduled_time: Option<String>,
  #[serde(rename = "ownerMembershipId")]
  owner_membership_id: Option<i64>,
  #[serde(rename = "playerSlotCount")]
  player_slot_count: Option<i32>,
  #[serde(rename = "alternateSlotCount")]
  alternate_slot_count: Option<i32>,
  #[serde(rename = "availablePlayerSlotCount")]
  available_player_slot_count: Option<i32>,
  #[serde(rename = "availableAlternateSlotCount")]
  available_alternate_slot_count: Option<i32>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "dateCreated")]
  date_created: Option<String>,
  #[serde(rename = "dateModified")]
  date_modified: Option<String>,
  #[serde(rename = "isPublic")]
  is_public: Option<bool>,
  #[serde(rename = "locale")]
  locale: Option<String>,
  #[serde(rename = "isValid")]
  is_valid: Option<bool>,
  #[serde(rename = "datePlayerModified")]
  date_player_modified: Option<String>,
  #[serde(rename = "titleBeforeModeration")]
  title_before_moderation: Option<String>
}

impl FireteamFireteamSummary {
  pub fn new() -> FireteamFireteamSummary {
    FireteamFireteamSummary {
      fireteam_id: None,
      group_id: None,
      platform: None,
      activity_type: None,
      is_immediate: None,
      scheduled_time: None,
      owner_membership_id: None,
      player_slot_count: None,
      alternate_slot_count: None,
      available_player_slot_count: None,
      available_alternate_slot_count: None,
      title: None,
      date_created: None,
      date_modified: None,
      is_public: None,
      locale: None,
      is_valid: None,
      date_player_modified: None,
      title_before_moderation: None
    }
  }

  pub fn set_fireteam_id(&mut self, fireteam_id: i64) {
    self.fireteam_id = Some(fireteam_id);
  }

  pub fn with_fireteam_id(mut self, fireteam_id: i64) -> FireteamFireteamSummary {
    self.fireteam_id = Some(fireteam_id);
    self
  }

  pub fn fireteam_id(&self) -> Option<&i64> {
    self.fireteam_id.as_ref()
  }

  pub fn reset_fireteam_id(&mut self) {
    self.fireteam_id = None;
  }

  pub fn set_group_id(&mut self, group_id: i64) {
    self.group_id = Some(group_id);
  }

  pub fn with_group_id(mut self, group_id: i64) -> FireteamFireteamSummary {
    self.group_id = Some(group_id);
    self
  }

  pub fn group_id(&self) -> Option<&i64> {
    self.group_id.as_ref()
  }

  pub fn reset_group_id(&mut self) {
    self.group_id = None;
  }

  pub fn set_platform(&mut self, platform: i32) {
    self.platform = Some(platform);
  }

  pub fn with_platform(mut self, platform: i32) -> FireteamFireteamSummary {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> Option<&i32> {
    self.platform.as_ref()
  }

  pub fn reset_platform(&mut self) {
    self.platform = None;
  }

  pub fn set_activity_type(&mut self, activity_type: i32) {
    self.activity_type = Some(activity_type);
  }

  pub fn with_activity_type(mut self, activity_type: i32) -> FireteamFireteamSummary {
    self.activity_type = Some(activity_type);
    self
  }

  pub fn activity_type(&self) -> Option<&i32> {
    self.activity_type.as_ref()
  }

  pub fn reset_activity_type(&mut self) {
    self.activity_type = None;
  }

  pub fn set_is_immediate(&mut self, is_immediate: bool) {
    self.is_immediate = Some(is_immediate);
  }

  pub fn with_is_immediate(mut self, is_immediate: bool) -> FireteamFireteamSummary {
    self.is_immediate = Some(is_immediate);
    self
  }

  pub fn is_immediate(&self) -> Option<&bool> {
    self.is_immediate.as_ref()
  }

  pub fn reset_is_immediate(&mut self) {
    self.is_immediate = None;
  }

  pub fn set_scheduled_time(&mut self, scheduled_time: String) {
    self.scheduled_time = Some(scheduled_time);
  }

  pub fn with_scheduled_time(mut self, scheduled_time: String) -> FireteamFireteamSummary {
    self.scheduled_time = Some(scheduled_time);
    self
  }

  pub fn scheduled_time(&self) -> Option<&String> {
    self.scheduled_time.as_ref()
  }

  pub fn reset_scheduled_time(&mut self) {
    self.scheduled_time = None;
  }

  pub fn set_owner_membership_id(&mut self, owner_membership_id: i64) {
    self.owner_membership_id = Some(owner_membership_id);
  }

  pub fn with_owner_membership_id(mut self, owner_membership_id: i64) -> FireteamFireteamSummary {
    self.owner_membership_id = Some(owner_membership_id);
    self
  }

  pub fn owner_membership_id(&self) -> Option<&i64> {
    self.owner_membership_id.as_ref()
  }

  pub fn reset_owner_membership_id(&mut self) {
    self.owner_membership_id = None;
  }

  pub fn set_player_slot_count(&mut self, player_slot_count: i32) {
    self.player_slot_count = Some(player_slot_count);
  }

  pub fn with_player_slot_count(mut self, player_slot_count: i32) -> FireteamFireteamSummary {
    self.player_slot_count = Some(player_slot_count);
    self
  }

  pub fn player_slot_count(&self) -> Option<&i32> {
    self.player_slot_count.as_ref()
  }

  pub fn reset_player_slot_count(&mut self) {
    self.player_slot_count = None;
  }

  pub fn set_alternate_slot_count(&mut self, alternate_slot_count: i32) {
    self.alternate_slot_count = Some(alternate_slot_count);
  }

  pub fn with_alternate_slot_count(mut self, alternate_slot_count: i32) -> FireteamFireteamSummary {
    self.alternate_slot_count = Some(alternate_slot_count);
    self
  }

  pub fn alternate_slot_count(&self) -> Option<&i32> {
    self.alternate_slot_count.as_ref()
  }

  pub fn reset_alternate_slot_count(&mut self) {
    self.alternate_slot_count = None;
  }

  pub fn set_available_player_slot_count(&mut self, available_player_slot_count: i32) {
    self.available_player_slot_count = Some(available_player_slot_count);
  }

  pub fn with_available_player_slot_count(mut self, available_player_slot_count: i32) -> FireteamFireteamSummary {
    self.available_player_slot_count = Some(available_player_slot_count);
    self
  }

  pub fn available_player_slot_count(&self) -> Option<&i32> {
    self.available_player_slot_count.as_ref()
  }

  pub fn reset_available_player_slot_count(&mut self) {
    self.available_player_slot_count = None;
  }

  pub fn set_available_alternate_slot_count(&mut self, available_alternate_slot_count: i32) {
    self.available_alternate_slot_count = Some(available_alternate_slot_count);
  }

  pub fn with_available_alternate_slot_count(mut self, available_alternate_slot_count: i32) -> FireteamFireteamSummary {
    self.available_alternate_slot_count = Some(available_alternate_slot_count);
    self
  }

  pub fn available_alternate_slot_count(&self) -> Option<&i32> {
    self.available_alternate_slot_count.as_ref()
  }

  pub fn reset_available_alternate_slot_count(&mut self) {
    self.available_alternate_slot_count = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> FireteamFireteamSummary {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_date_created(&mut self, date_created: String) {
    self.date_created = Some(date_created);
  }

  pub fn with_date_created(mut self, date_created: String) -> FireteamFireteamSummary {
    self.date_created = Some(date_created);
    self
  }

  pub fn date_created(&self) -> Option<&String> {
    self.date_created.as_ref()
  }

  pub fn reset_date_created(&mut self) {
    self.date_created = None;
  }

  pub fn set_date_modified(&mut self, date_modified: String) {
    self.date_modified = Some(date_modified);
  }

  pub fn with_date_modified(mut self, date_modified: String) -> FireteamFireteamSummary {
    self.date_modified = Some(date_modified);
    self
  }

  pub fn date_modified(&self) -> Option<&String> {
    self.date_modified.as_ref()
  }

  pub fn reset_date_modified(&mut self) {
    self.date_modified = None;
  }

  pub fn set_is_public(&mut self, is_public: bool) {
    self.is_public = Some(is_public);
  }

  pub fn with_is_public(mut self, is_public: bool) -> FireteamFireteamSummary {
    self.is_public = Some(is_public);
    self
  }

  pub fn is_public(&self) -> Option<&bool> {
    self.is_public.as_ref()
  }

  pub fn reset_is_public(&mut self) {
    self.is_public = None;
  }

  pub fn set_locale(&mut self, locale: String) {
    self.locale = Some(locale);
  }

  pub fn with_locale(mut self, locale: String) -> FireteamFireteamSummary {
    self.locale = Some(locale);
    self
  }

  pub fn locale(&self) -> Option<&String> {
    self.locale.as_ref()
  }

  pub fn reset_locale(&mut self) {
    self.locale = None;
  }

  pub fn set_is_valid(&mut self, is_valid: bool) {
    self.is_valid = Some(is_valid);
  }

  pub fn with_is_valid(mut self, is_valid: bool) -> FireteamFireteamSummary {
    self.is_valid = Some(is_valid);
    self
  }

  pub fn is_valid(&self) -> Option<&bool> {
    self.is_valid.as_ref()
  }

  pub fn reset_is_valid(&mut self) {
    self.is_valid = None;
  }

  pub fn set_date_player_modified(&mut self, date_player_modified: String) {
    self.date_player_modified = Some(date_player_modified);
  }

  pub fn with_date_player_modified(mut self, date_player_modified: String) -> FireteamFireteamSummary {
    self.date_player_modified = Some(date_player_modified);
    self
  }

  pub fn date_player_modified(&self) -> Option<&String> {
    self.date_player_modified.as_ref()
  }

  pub fn reset_date_player_modified(&mut self) {
    self.date_player_modified = None;
  }

  pub fn set_title_before_moderation(&mut self, title_before_moderation: String) {
    self.title_before_moderation = Some(title_before_moderation);
  }

  pub fn with_title_before_moderation(mut self, title_before_moderation: String) -> FireteamFireteamSummary {
    self.title_before_moderation = Some(title_before_moderation);
    self
  }

  pub fn title_before_moderation(&self) -> Option<&String> {
    self.title_before_moderation.as_ref()
  }

  pub fn reset_title_before_moderation(&mut self) {
    self.title_before_moderation = None;
  }

}



