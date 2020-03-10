/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition : Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
  /// The description of the energy type, icon etc...
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  /// A variant of the icon that is transparent and colorless.
  #[serde(rename = "transparentIconPath")]
  transparent_icon_path: Option<String>,
  /// If TRUE, the game shows this Energy type's icon. Otherwise, it doesn't. Whether you show it or not is up to you.
  #[serde(rename = "showIcon")]
  show_icon: Option<bool>,
  /// We have an enumeration for Energy types for quick reference. This is the current definition's Energy type enum value.
  #[serde(rename = "enumValue")]
  enum_value: Option<i32>,
  /// If this Energy Type can be used for determining the Type of Energy that an item can consume, this is the hash for the DestinyInvestmentStatDefinition that represents the stat which holds the Capacity for that energy type. (Note that this is optional because \"Any\" is a valid cost, but not valid for Capacity - an Armor must have a specific Energy Type for determining the energy type that the Armor is restricted to use)
  #[serde(rename = "capacityStatHash")]
  capacity_stat_hash: Option<i32>,
  /// If this Energy Type can be used as a cost to pay for socketing Armor 2.0 items, this is the hash for the DestinyInvestmentStatDefinition that stores the plug's raw cost.
  #[serde(rename = "costStatHash")]
  cost_stat_hash: Option<i32>,
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

impl DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
  /// Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.
  pub fn new() -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
      display_properties: None,
      transparent_icon_path: None,
      show_icon: None,
      enum_value: None,
      capacity_stat_hash: None,
      cost_stat_hash: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_transparent_icon_path(&mut self, transparent_icon_path: String) {
    self.transparent_icon_path = Some(transparent_icon_path);
  }

  pub fn with_transparent_icon_path(mut self, transparent_icon_path: String) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.transparent_icon_path = Some(transparent_icon_path);
    self
  }

  pub fn transparent_icon_path(&self) -> Option<&String> {
    self.transparent_icon_path.as_ref()
  }

  pub fn reset_transparent_icon_path(&mut self) {
    self.transparent_icon_path = None;
  }

  pub fn set_show_icon(&mut self, show_icon: bool) {
    self.show_icon = Some(show_icon);
  }

  pub fn with_show_icon(mut self, show_icon: bool) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.show_icon = Some(show_icon);
    self
  }

  pub fn show_icon(&self) -> Option<&bool> {
    self.show_icon.as_ref()
  }

  pub fn reset_show_icon(&mut self) {
    self.show_icon = None;
  }

  pub fn set_enum_value(&mut self, enum_value: i32) {
    self.enum_value = Some(enum_value);
  }

  pub fn with_enum_value(mut self, enum_value: i32) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.enum_value = Some(enum_value);
    self
  }

  pub fn enum_value(&self) -> Option<&i32> {
    self.enum_value.as_ref()
  }

  pub fn reset_enum_value(&mut self) {
    self.enum_value = None;
  }

  pub fn set_capacity_stat_hash(&mut self, capacity_stat_hash: i32) {
    self.capacity_stat_hash = Some(capacity_stat_hash);
  }

  pub fn with_capacity_stat_hash(mut self, capacity_stat_hash: i32) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.capacity_stat_hash = Some(capacity_stat_hash);
    self
  }

  pub fn capacity_stat_hash(&self) -> Option<&i32> {
    self.capacity_stat_hash.as_ref()
  }

  pub fn reset_capacity_stat_hash(&mut self) {
    self.capacity_stat_hash = None;
  }

  pub fn set_cost_stat_hash(&mut self, cost_stat_hash: i32) {
    self.cost_stat_hash = Some(cost_stat_hash);
  }

  pub fn with_cost_stat_hash(mut self, cost_stat_hash: i32) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
    self.cost_stat_hash = Some(cost_stat_hash);
    self
  }

  pub fn cost_stat_hash(&self) -> Option<&i32> {
    self.cost_stat_hash.as_ref()
  }

  pub fn reset_cost_stat_hash(&mut self) {
    self.cost_stat_hash = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
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

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
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

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinition {
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



