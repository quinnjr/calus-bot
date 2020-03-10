/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TrendingTrendingEntry : The list entry view for trending items. Returns just enough to show the item on the trending page.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingTrendingEntry {
  /// The weighted score of this trending item.
  #[serde(rename = "weight")]
  weight: Option<f64>,
  #[serde(rename = "isFeatured")]
  is_featured: Option<bool>,
  /// We don't know whether the identifier will be a string, a uint, or a long... so we're going to cast it all to a string. But either way, we need any trending item created to have a single unique identifier for its type.
  #[serde(rename = "identifier")]
  identifier: Option<String>,
  /// An enum - unfortunately - dictating all of the possible kinds of trending items that you might get in your result set, in case you want to do custom rendering or call to get the details of the item.
  #[serde(rename = "entityType")]
  entity_type: Option<i32>,
  /// The localized \"display name/article title/'primary localized identifier'\" of the entity.
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  /// If the entity has a localized tagline/subtitle/motto/whatever, that is found here.
  #[serde(rename = "tagline")]
  tagline: Option<String>,
  #[serde(rename = "image")]
  image: Option<String>,
  #[serde(rename = "startDate")]
  start_date: Option<String>,
  #[serde(rename = "endDate")]
  end_date: Option<String>,
  #[serde(rename = "link")]
  link: Option<String>,
  /// If this is populated, the entry has a related WebM video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
  #[serde(rename = "webmVideo")]
  webm_video: Option<String>,
  /// If this is populated, the entry has a related MP4 video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
  #[serde(rename = "mp4Video")]
  mp4_video: Option<String>,
  /// If isFeatured, this image will be populated with whatever the featured image is. Note that this will likely be a very large image, so don't use it all the time.
  #[serde(rename = "featureImage")]
  feature_image: Option<String>,
  /// If the item is of entityType TrendingEntryType.Container, it may have items - also Trending Entries - contained within it. This is the ordered list of those to display under the Container's header.
  #[serde(rename = "items")]
  items: Option<Vec<::models::TrendingTrendingEntry>>,
  /// If the entry has a date at which it was created, this is that date.
  #[serde(rename = "creationDate")]
  creation_date: Option<String>
}

impl TrendingTrendingEntry {
  /// The list entry view for trending items. Returns just enough to show the item on the trending page.
  pub fn new() -> TrendingTrendingEntry {
    TrendingTrendingEntry {
      weight: None,
      is_featured: None,
      identifier: None,
      entity_type: None,
      display_name: None,
      tagline: None,
      image: None,
      start_date: None,
      end_date: None,
      link: None,
      webm_video: None,
      mp4_video: None,
      feature_image: None,
      items: None,
      creation_date: None
    }
  }

  pub fn set_weight(&mut self, weight: f64) {
    self.weight = Some(weight);
  }

  pub fn with_weight(mut self, weight: f64) -> TrendingTrendingEntry {
    self.weight = Some(weight);
    self
  }

  pub fn weight(&self) -> Option<&f64> {
    self.weight.as_ref()
  }

  pub fn reset_weight(&mut self) {
    self.weight = None;
  }

  pub fn set_is_featured(&mut self, is_featured: bool) {
    self.is_featured = Some(is_featured);
  }

  pub fn with_is_featured(mut self, is_featured: bool) -> TrendingTrendingEntry {
    self.is_featured = Some(is_featured);
    self
  }

  pub fn is_featured(&self) -> Option<&bool> {
    self.is_featured.as_ref()
  }

  pub fn reset_is_featured(&mut self) {
    self.is_featured = None;
  }

  pub fn set_identifier(&mut self, identifier: String) {
    self.identifier = Some(identifier);
  }

  pub fn with_identifier(mut self, identifier: String) -> TrendingTrendingEntry {
    self.identifier = Some(identifier);
    self
  }

  pub fn identifier(&self) -> Option<&String> {
    self.identifier.as_ref()
  }

  pub fn reset_identifier(&mut self) {
    self.identifier = None;
  }

  pub fn set_entity_type(&mut self, entity_type: i32) {
    self.entity_type = Some(entity_type);
  }

  pub fn with_entity_type(mut self, entity_type: i32) -> TrendingTrendingEntry {
    self.entity_type = Some(entity_type);
    self
  }

  pub fn entity_type(&self) -> Option<&i32> {
    self.entity_type.as_ref()
  }

  pub fn reset_entity_type(&mut self) {
    self.entity_type = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> TrendingTrendingEntry {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_tagline(&mut self, tagline: String) {
    self.tagline = Some(tagline);
  }

  pub fn with_tagline(mut self, tagline: String) -> TrendingTrendingEntry {
    self.tagline = Some(tagline);
    self
  }

  pub fn tagline(&self) -> Option<&String> {
    self.tagline.as_ref()
  }

  pub fn reset_tagline(&mut self) {
    self.tagline = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = Some(image);
  }

  pub fn with_image(mut self, image: String) -> TrendingTrendingEntry {
    self.image = Some(image);
    self
  }

  pub fn image(&self) -> Option<&String> {
    self.image.as_ref()
  }

  pub fn reset_image(&mut self) {
    self.image = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> TrendingTrendingEntry {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = Some(end_date);
  }

  pub fn with_end_date(mut self, end_date: String) -> TrendingTrendingEntry {
    self.end_date = Some(end_date);
    self
  }

  pub fn end_date(&self) -> Option<&String> {
    self.end_date.as_ref()
  }

  pub fn reset_end_date(&mut self) {
    self.end_date = None;
  }

  pub fn set_link(&mut self, link: String) {
    self.link = Some(link);
  }

  pub fn with_link(mut self, link: String) -> TrendingTrendingEntry {
    self.link = Some(link);
    self
  }

  pub fn link(&self) -> Option<&String> {
    self.link.as_ref()
  }

  pub fn reset_link(&mut self) {
    self.link = None;
  }

  pub fn set_webm_video(&mut self, webm_video: String) {
    self.webm_video = Some(webm_video);
  }

  pub fn with_webm_video(mut self, webm_video: String) -> TrendingTrendingEntry {
    self.webm_video = Some(webm_video);
    self
  }

  pub fn webm_video(&self) -> Option<&String> {
    self.webm_video.as_ref()
  }

  pub fn reset_webm_video(&mut self) {
    self.webm_video = None;
  }

  pub fn set_mp4_video(&mut self, mp4_video: String) {
    self.mp4_video = Some(mp4_video);
  }

  pub fn with_mp4_video(mut self, mp4_video: String) -> TrendingTrendingEntry {
    self.mp4_video = Some(mp4_video);
    self
  }

  pub fn mp4_video(&self) -> Option<&String> {
    self.mp4_video.as_ref()
  }

  pub fn reset_mp4_video(&mut self) {
    self.mp4_video = None;
  }

  pub fn set_feature_image(&mut self, feature_image: String) {
    self.feature_image = Some(feature_image);
  }

  pub fn with_feature_image(mut self, feature_image: String) -> TrendingTrendingEntry {
    self.feature_image = Some(feature_image);
    self
  }

  pub fn feature_image(&self) -> Option<&String> {
    self.feature_image.as_ref()
  }

  pub fn reset_feature_image(&mut self) {
    self.feature_image = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::TrendingTrendingEntry>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::TrendingTrendingEntry>) -> TrendingTrendingEntry {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::TrendingTrendingEntry>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_creation_date(&mut self, creation_date: String) {
    self.creation_date = Some(creation_date);
  }

  pub fn with_creation_date(mut self, creation_date: String) -> TrendingTrendingEntry {
    self.creation_date = Some(creation_date);
    self
  }

  pub fn creation_date(&self) -> Option<&String> {
    self.creation_date.as_ref()
  }

  pub fn reset_creation_date(&mut self) {
    self.creation_date = None;
  }

}



