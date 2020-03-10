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
pub struct DestinyConstantsDestinyEnvironmentLocationMapping {
  /// The location that is revealed on the director by this mapping.
  #[serde(rename = "locationHash")]
  location_hash: Option<i32>,
  /// A hint that the UI uses to figure out how this location is activated by the player.
  #[serde(rename = "activationSource")]
  activation_source: Option<String>,
  /// If this is populated, it is the item that you must possess for this location to be active because of this mapping. (theoretically, a location can have multiple mappings, and some might require an item while others don't)
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  /// If this is populated, this is an objective related to the location.
  #[serde(rename = "objectiveHash")]
  objective_hash: Option<i32>,
  /// If this is populated, this is the activity you have to be playing in order to see this location appear because of this mapping. (theoretically, a location can have multiple mappings, and some might require you to be in a specific activity when others don't)
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>
}

impl DestinyConstantsDestinyEnvironmentLocationMapping {
  pub fn new() -> DestinyConstantsDestinyEnvironmentLocationMapping {
    DestinyConstantsDestinyEnvironmentLocationMapping {
      location_hash: None,
      activation_source: None,
      item_hash: None,
      objective_hash: None,
      activity_hash: None
    }
  }

  pub fn set_location_hash(&mut self, location_hash: i32) {
    self.location_hash = Some(location_hash);
  }

  pub fn with_location_hash(mut self, location_hash: i32) -> DestinyConstantsDestinyEnvironmentLocationMapping {
    self.location_hash = Some(location_hash);
    self
  }

  pub fn location_hash(&self) -> Option<&i32> {
    self.location_hash.as_ref()
  }

  pub fn reset_location_hash(&mut self) {
    self.location_hash = None;
  }

  pub fn set_activation_source(&mut self, activation_source: String) {
    self.activation_source = Some(activation_source);
  }

  pub fn with_activation_source(mut self, activation_source: String) -> DestinyConstantsDestinyEnvironmentLocationMapping {
    self.activation_source = Some(activation_source);
    self
  }

  pub fn activation_source(&self) -> Option<&String> {
    self.activation_source.as_ref()
  }

  pub fn reset_activation_source(&mut self) {
    self.activation_source = None;
  }

  pub fn set_item_hash(&mut self, item_hash: i32) {
    self.item_hash = Some(item_hash);
  }

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyConstantsDestinyEnvironmentLocationMapping {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_objective_hash(&mut self, objective_hash: i32) {
    self.objective_hash = Some(objective_hash);
  }

  pub fn with_objective_hash(mut self, objective_hash: i32) -> DestinyConstantsDestinyEnvironmentLocationMapping {
    self.objective_hash = Some(objective_hash);
    self
  }

  pub fn objective_hash(&self) -> Option<&i32> {
    self.objective_hash.as_ref()
  }

  pub fn reset_objective_hash(&mut self) {
    self.objective_hash = None;
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyConstantsDestinyEnvironmentLocationMapping {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

}


