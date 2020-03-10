/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition : The properties of an individual checklist item. Note that almost everything is optional: it is *highly* variable what kind of data we'll actually be able to return: at times we may have no other relationships to entities at all.  Whatever UI you build, do it with the knowledge that any given entry might not actually be able to be associated with some other Destiny entity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
  /// The identifier for this Checklist entry. Guaranteed unique only within this Checklist Definition, and not globally/for all checklists.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// Even if no other associations exist, we will give you *something* for display properties. In cases where we have no associated entities, it may be as simple as a numerical identifier.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  #[serde(rename = "destinationHash")]
  destination_hash: Option<i32>,
  #[serde(rename = "locationHash")]
  location_hash: Option<i32>,
  /// Note that a Bubble's hash doesn't uniquely identify a \"top level\" entity in Destiny. Only the combination of location and bubble can uniquely identify a place in the world of Destiny: so if bubbleHash is populated, locationHash must too be populated for it to have any meaning.  You can use this property if it is populated to look up the DestinyLocationDefinition's associated .locationReleases[].activityBubbleName property.
  #[serde(rename = "bubbleHash")]
  bubble_hash: Option<i32>,
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  #[serde(rename = "vendorHash")]
  vendor_hash: Option<i32>,
  #[serde(rename = "vendorInteractionIndex")]
  vendor_interaction_index: Option<i32>,
  /// The scope at which this specific entry can be computed.
  #[serde(rename = "scope")]
  scope: Option<i32>
}

impl DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
  /// The properties of an individual checklist item. Note that almost everything is optional: it is *highly* variable what kind of data we'll actually be able to return: at times we may have no other relationships to entities at all.  Whatever UI you build, do it with the knowledge that any given entry might not actually be able to be associated with some other Destiny entity.
  pub fn new() -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
      hash: None,
      display_properties: None,
      destination_hash: None,
      location_hash: None,
      bubble_hash: None,
      activity_hash: None,
      item_hash: None,
      vendor_hash: None,
      vendor_interaction_index: None,
      scope: None
    }
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_destination_hash(&mut self, destination_hash: i32) {
    self.destination_hash = Some(destination_hash);
  }

  pub fn with_destination_hash(mut self, destination_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.destination_hash = Some(destination_hash);
    self
  }

  pub fn destination_hash(&self) -> Option<&i32> {
    self.destination_hash.as_ref()
  }

  pub fn reset_destination_hash(&mut self) {
    self.destination_hash = None;
  }

  pub fn set_location_hash(&mut self, location_hash: i32) {
    self.location_hash = Some(location_hash);
  }

  pub fn with_location_hash(mut self, location_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.location_hash = Some(location_hash);
    self
  }

  pub fn location_hash(&self) -> Option<&i32> {
    self.location_hash.as_ref()
  }

  pub fn reset_location_hash(&mut self) {
    self.location_hash = None;
  }

  pub fn set_bubble_hash(&mut self, bubble_hash: i32) {
    self.bubble_hash = Some(bubble_hash);
  }

  pub fn with_bubble_hash(mut self, bubble_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.bubble_hash = Some(bubble_hash);
    self
  }

  pub fn bubble_hash(&self) -> Option<&i32> {
    self.bubble_hash.as_ref()
  }

  pub fn reset_bubble_hash(&mut self) {
    self.bubble_hash = None;
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_item_hash(&mut self, item_hash: i32) {
    self.item_hash = Some(item_hash);
  }

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_vendor_hash(&mut self, vendor_hash: i32) {
    self.vendor_hash = Some(vendor_hash);
  }

  pub fn with_vendor_hash(mut self, vendor_hash: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.vendor_hash = Some(vendor_hash);
    self
  }

  pub fn vendor_hash(&self) -> Option<&i32> {
    self.vendor_hash.as_ref()
  }

  pub fn reset_vendor_hash(&mut self) {
    self.vendor_hash = None;
  }

  pub fn set_vendor_interaction_index(&mut self, vendor_interaction_index: i32) {
    self.vendor_interaction_index = Some(vendor_interaction_index);
  }

  pub fn with_vendor_interaction_index(mut self, vendor_interaction_index: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.vendor_interaction_index = Some(vendor_interaction_index);
    self
  }

  pub fn vendor_interaction_index(&self) -> Option<&i32> {
    self.vendor_interaction_index.as_ref()
  }

  pub fn reset_vendor_interaction_index(&mut self) {
    self.vendor_interaction_index = None;
  }

  pub fn set_scope(&mut self, scope: i32) {
    self.scope = Some(scope);
  }

  pub fn with_scope(mut self, scope: i32) -> DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition {
    self.scope = Some(scope);
    self
  }

  pub fn scope(&self) -> Option<&i32> {
    self.scope.as_ref()
  }

  pub fn reset_scope(&mut self) {
    self.scope = None;
  }

}



