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
pub struct CommunityCommunityLiveStatus {
  #[serde(rename = "dateStatusUpdated")]
  date_status_updated: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "partnershipIdentifier")]
  partnership_identifier: Option<String>,
  #[serde(rename = "partnershipType")]
  partnership_type: Option<i32>,
  #[serde(rename = "thumbnail")]
  thumbnail: Option<String>,
  #[serde(rename = "thumbnailSmall")]
  thumbnail_small: Option<String>,
  #[serde(rename = "thumbnailLarge")]
  thumbnail_large: Option<String>,
  #[serde(rename = "destinyCharacterId")]
  destiny_character_id: Option<i64>,
  #[serde(rename = "userInfo")]
  user_info: Option<::models::UserUserInfoCard>,
  #[serde(rename = "currentActivityHash")]
  current_activity_hash: Option<i32>,
  #[serde(rename = "dateLastPlayed")]
  date_last_played: Option<String>,
  #[serde(rename = "dateStreamStarted")]
  date_stream_started: Option<String>,
  #[serde(rename = "locale")]
  locale: Option<String>,
  #[serde(rename = "currentViewers")]
  current_viewers: Option<i32>,
  #[serde(rename = "followers")]
  followers: Option<i32>,
  #[serde(rename = "overallViewers")]
  overall_viewers: Option<i32>,
  #[serde(rename = "isFeatured")]
  is_featured: Option<bool>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "activityModeHash")]
  activity_mode_hash: Option<i32>,
  #[serde(rename = "dateFeatured")]
  date_featured: Option<String>,
  #[serde(rename = "trendingValue")]
  trending_value: Option<f32>,
  #[serde(rename = "isSubscribable")]
  is_subscribable: Option<bool>
}

impl CommunityCommunityLiveStatus {
  pub fn new() -> CommunityCommunityLiveStatus {
    CommunityCommunityLiveStatus {
      date_status_updated: None,
      url: None,
      partnership_identifier: None,
      partnership_type: None,
      thumbnail: None,
      thumbnail_small: None,
      thumbnail_large: None,
      destiny_character_id: None,
      user_info: None,
      current_activity_hash: None,
      date_last_played: None,
      date_stream_started: None,
      locale: None,
      current_viewers: None,
      followers: None,
      overall_viewers: None,
      is_featured: None,
      title: None,
      activity_mode_hash: None,
      date_featured: None,
      trending_value: None,
      is_subscribable: None
    }
  }

  pub fn set_date_status_updated(&mut self, date_status_updated: String) {
    self.date_status_updated = Some(date_status_updated);
  }

  pub fn with_date_status_updated(mut self, date_status_updated: String) -> CommunityCommunityLiveStatus {
    self.date_status_updated = Some(date_status_updated);
    self
  }

  pub fn date_status_updated(&self) -> Option<&String> {
    self.date_status_updated.as_ref()
  }

  pub fn reset_date_status_updated(&mut self) {
    self.date_status_updated = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> CommunityCommunityLiveStatus {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_partnership_identifier(&mut self, partnership_identifier: String) {
    self.partnership_identifier = Some(partnership_identifier);
  }

  pub fn with_partnership_identifier(mut self, partnership_identifier: String) -> CommunityCommunityLiveStatus {
    self.partnership_identifier = Some(partnership_identifier);
    self
  }

  pub fn partnership_identifier(&self) -> Option<&String> {
    self.partnership_identifier.as_ref()
  }

  pub fn reset_partnership_identifier(&mut self) {
    self.partnership_identifier = None;
  }

  pub fn set_partnership_type(&mut self, partnership_type: i32) {
    self.partnership_type = Some(partnership_type);
  }

  pub fn with_partnership_type(mut self, partnership_type: i32) -> CommunityCommunityLiveStatus {
    self.partnership_type = Some(partnership_type);
    self
  }

  pub fn partnership_type(&self) -> Option<&i32> {
    self.partnership_type.as_ref()
  }

  pub fn reset_partnership_type(&mut self) {
    self.partnership_type = None;
  }

  pub fn set_thumbnail(&mut self, thumbnail: String) {
    self.thumbnail = Some(thumbnail);
  }

  pub fn with_thumbnail(mut self, thumbnail: String) -> CommunityCommunityLiveStatus {
    self.thumbnail = Some(thumbnail);
    self
  }

  pub fn thumbnail(&self) -> Option<&String> {
    self.thumbnail.as_ref()
  }

  pub fn reset_thumbnail(&mut self) {
    self.thumbnail = None;
  }

  pub fn set_thumbnail_small(&mut self, thumbnail_small: String) {
    self.thumbnail_small = Some(thumbnail_small);
  }

  pub fn with_thumbnail_small(mut self, thumbnail_small: String) -> CommunityCommunityLiveStatus {
    self.thumbnail_small = Some(thumbnail_small);
    self
  }

  pub fn thumbnail_small(&self) -> Option<&String> {
    self.thumbnail_small.as_ref()
  }

  pub fn reset_thumbnail_small(&mut self) {
    self.thumbnail_small = None;
  }

  pub fn set_thumbnail_large(&mut self, thumbnail_large: String) {
    self.thumbnail_large = Some(thumbnail_large);
  }

  pub fn with_thumbnail_large(mut self, thumbnail_large: String) -> CommunityCommunityLiveStatus {
    self.thumbnail_large = Some(thumbnail_large);
    self
  }

  pub fn thumbnail_large(&self) -> Option<&String> {
    self.thumbnail_large.as_ref()
  }

  pub fn reset_thumbnail_large(&mut self) {
    self.thumbnail_large = None;
  }

  pub fn set_destiny_character_id(&mut self, destiny_character_id: i64) {
    self.destiny_character_id = Some(destiny_character_id);
  }

  pub fn with_destiny_character_id(mut self, destiny_character_id: i64) -> CommunityCommunityLiveStatus {
    self.destiny_character_id = Some(destiny_character_id);
    self
  }

  pub fn destiny_character_id(&self) -> Option<&i64> {
    self.destiny_character_id.as_ref()
  }

  pub fn reset_destiny_character_id(&mut self) {
    self.destiny_character_id = None;
  }

  pub fn set_user_info(&mut self, user_info: ::models::UserUserInfoCard) {
    self.user_info = Some(user_info);
  }

  pub fn with_user_info(mut self, user_info: ::models::UserUserInfoCard) -> CommunityCommunityLiveStatus {
    self.user_info = Some(user_info);
    self
  }

  pub fn user_info(&self) -> Option<&::models::UserUserInfoCard> {
    self.user_info.as_ref()
  }

  pub fn reset_user_info(&mut self) {
    self.user_info = None;
  }

  pub fn set_current_activity_hash(&mut self, current_activity_hash: i32) {
    self.current_activity_hash = Some(current_activity_hash);
  }

  pub fn with_current_activity_hash(mut self, current_activity_hash: i32) -> CommunityCommunityLiveStatus {
    self.current_activity_hash = Some(current_activity_hash);
    self
  }

  pub fn current_activity_hash(&self) -> Option<&i32> {
    self.current_activity_hash.as_ref()
  }

  pub fn reset_current_activity_hash(&mut self) {
    self.current_activity_hash = None;
  }

  pub fn set_date_last_played(&mut self, date_last_played: String) {
    self.date_last_played = Some(date_last_played);
  }

  pub fn with_date_last_played(mut self, date_last_played: String) -> CommunityCommunityLiveStatus {
    self.date_last_played = Some(date_last_played);
    self
  }

  pub fn date_last_played(&self) -> Option<&String> {
    self.date_last_played.as_ref()
  }

  pub fn reset_date_last_played(&mut self) {
    self.date_last_played = None;
  }

  pub fn set_date_stream_started(&mut self, date_stream_started: String) {
    self.date_stream_started = Some(date_stream_started);
  }

  pub fn with_date_stream_started(mut self, date_stream_started: String) -> CommunityCommunityLiveStatus {
    self.date_stream_started = Some(date_stream_started);
    self
  }

  pub fn date_stream_started(&self) -> Option<&String> {
    self.date_stream_started.as_ref()
  }

  pub fn reset_date_stream_started(&mut self) {
    self.date_stream_started = None;
  }

  pub fn set_locale(&mut self, locale: String) {
    self.locale = Some(locale);
  }

  pub fn with_locale(mut self, locale: String) -> CommunityCommunityLiveStatus {
    self.locale = Some(locale);
    self
  }

  pub fn locale(&self) -> Option<&String> {
    self.locale.as_ref()
  }

  pub fn reset_locale(&mut self) {
    self.locale = None;
  }

  pub fn set_current_viewers(&mut self, current_viewers: i32) {
    self.current_viewers = Some(current_viewers);
  }

  pub fn with_current_viewers(mut self, current_viewers: i32) -> CommunityCommunityLiveStatus {
    self.current_viewers = Some(current_viewers);
    self
  }

  pub fn current_viewers(&self) -> Option<&i32> {
    self.current_viewers.as_ref()
  }

  pub fn reset_current_viewers(&mut self) {
    self.current_viewers = None;
  }

  pub fn set_followers(&mut self, followers: i32) {
    self.followers = Some(followers);
  }

  pub fn with_followers(mut self, followers: i32) -> CommunityCommunityLiveStatus {
    self.followers = Some(followers);
    self
  }

  pub fn followers(&self) -> Option<&i32> {
    self.followers.as_ref()
  }

  pub fn reset_followers(&mut self) {
    self.followers = None;
  }

  pub fn set_overall_viewers(&mut self, overall_viewers: i32) {
    self.overall_viewers = Some(overall_viewers);
  }

  pub fn with_overall_viewers(mut self, overall_viewers: i32) -> CommunityCommunityLiveStatus {
    self.overall_viewers = Some(overall_viewers);
    self
  }

  pub fn overall_viewers(&self) -> Option<&i32> {
    self.overall_viewers.as_ref()
  }

  pub fn reset_overall_viewers(&mut self) {
    self.overall_viewers = None;
  }

  pub fn set_is_featured(&mut self, is_featured: bool) {
    self.is_featured = Some(is_featured);
  }

  pub fn with_is_featured(mut self, is_featured: bool) -> CommunityCommunityLiveStatus {
    self.is_featured = Some(is_featured);
    self
  }

  pub fn is_featured(&self) -> Option<&bool> {
    self.is_featured.as_ref()
  }

  pub fn reset_is_featured(&mut self) {
    self.is_featured = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> CommunityCommunityLiveStatus {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_activity_mode_hash(&mut self, activity_mode_hash: i32) {
    self.activity_mode_hash = Some(activity_mode_hash);
  }

  pub fn with_activity_mode_hash(mut self, activity_mode_hash: i32) -> CommunityCommunityLiveStatus {
    self.activity_mode_hash = Some(activity_mode_hash);
    self
  }

  pub fn activity_mode_hash(&self) -> Option<&i32> {
    self.activity_mode_hash.as_ref()
  }

  pub fn reset_activity_mode_hash(&mut self) {
    self.activity_mode_hash = None;
  }

  pub fn set_date_featured(&mut self, date_featured: String) {
    self.date_featured = Some(date_featured);
  }

  pub fn with_date_featured(mut self, date_featured: String) -> CommunityCommunityLiveStatus {
    self.date_featured = Some(date_featured);
    self
  }

  pub fn date_featured(&self) -> Option<&String> {
    self.date_featured.as_ref()
  }

  pub fn reset_date_featured(&mut self) {
    self.date_featured = None;
  }

  pub fn set_trending_value(&mut self, trending_value: f32) {
    self.trending_value = Some(trending_value);
  }

  pub fn with_trending_value(mut self, trending_value: f32) -> CommunityCommunityLiveStatus {
    self.trending_value = Some(trending_value);
    self
  }

  pub fn trending_value(&self) -> Option<&f32> {
    self.trending_value.as_ref()
  }

  pub fn reset_trending_value(&mut self) {
    self.trending_value = None;
  }

  pub fn set_is_subscribable(&mut self, is_subscribable: bool) {
    self.is_subscribable = Some(is_subscribable);
  }

  pub fn with_is_subscribable(mut self, is_subscribable: bool) -> CommunityCommunityLiveStatus {
    self.is_subscribable = Some(is_subscribable);
    self
  }

  pub fn is_subscribable(&self) -> Option<&bool> {
    self.is_subscribable.as_ref()
  }

  pub fn reset_is_subscribable(&mut self) {
    self.is_subscribable = None;
  }

}



