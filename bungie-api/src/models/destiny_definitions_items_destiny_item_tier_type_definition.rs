/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsItemsDestinyItemTierTypeDefinition : Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...  It also provides some base data for infusion that could be useful.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// If this tier defines infusion properties, they will be contained here.
  #[serde(rename = "infusionProcess")]
  infusion_process: Option<Value>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
  /// Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...  It also provides some base data for infusion that could be useful.
  pub fn new() -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
      display_properties: None,
      infusion_process: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_infusion_process(&mut self, infusion_process: Value) {
    self.infusion_process = Some(infusion_process);
  }

  pub fn with_infusion_process(mut self, infusion_process: Value) -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    self.infusion_process = Some(infusion_process);
    self
  }

  pub fn infusion_process(&self) -> Option<&Value> {
    self.infusion_process.as_ref()
  }

  pub fn reset_infusion_process(&mut self) {
    self.infusion_process = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsItemsDestinyItemTierTypeDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}


