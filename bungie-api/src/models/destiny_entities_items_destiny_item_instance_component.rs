/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesItemsDestinyItemInstanceComponent : If an item is \"instanced\", this will contain information about the item's instance that doesn't fit easily into other components. One might say this is the \"essential\" instance data for the item.  Items are instanced if they require information or state that can vary. For instance, weapons are Instanced: they are given a unique identifier, uniquely generated stats, and can have their properties altered. Non-instanced items have none of these things: for instance, Glimmer has no unique properties aside from how much of it you own.  You can tell from an item's definition whether it will be instanced or not by looking at the DestinyInventoryItemDefinition's definition.inventory.isInstanceItem property.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesItemsDestinyItemInstanceComponent {
  /// If the item has a damage type, this is the item's current damage type.
  #[serde(rename = "damageType")]
  damage_type: Option<i32>,
  /// The current damage type's hash, so you can look up localized info and icons for it.
  #[serde(rename = "damageTypeHash")]
  damage_type_hash: Option<i32>,
  /// The item stat that we consider to be \"primary\" for the item. For instance, this would be \"Attack\" for Weapons or \"Defense\" for armor.
  #[serde(rename = "primaryStat")]
  primary_stat: Option<Value>,
  /// The Item's \"Level\" has the most significant bearing on its stats, such as Light and Power.
  #[serde(rename = "itemLevel")]
  item_level: Option<i32>,
  /// The \"Quality\" of the item has a lesser - but still impactful - bearing on stats like Light and Power.
  #[serde(rename = "quality")]
  quality: Option<i32>,
  /// Is the item currently equipped on the given character?
  #[serde(rename = "isEquipped")]
  is_equipped: Option<bool>,
  /// If this is an equippable item, you can check it here. There are permanent as well as transitory reasons why an item might not be able to be equipped: check cannotEquipReason for details.
  #[serde(rename = "canEquip")]
  can_equip: Option<bool>,
  /// If the item cannot be equipped until you reach a certain level, that level will be reflected here.
  #[serde(rename = "equipRequiredLevel")]
  equip_required_level: Option<i32>,
  /// Sometimes, there are limitations to equipping that are represented by character-level flags called \"unlocks\".  This is a list of flags that they need in order to equip the item that the character has not met. Use these to look up the descriptions to show in your UI by looking up the relevant DestinyUnlockDefinitions for the hashes.
  #[serde(rename = "unlockHashesRequiredToEquip")]
  unlock_hashes_required_to_equip: Option<Vec<i32>>,
  /// If you cannot equip the item, this is a flags enum that enumerates all of the reasons why you couldn't equip the item. You may need to refine your UI further by using unlockHashesRequiredToEquip and equipRequiredLevel.
  #[serde(rename = "cannotEquipReason")]
  cannot_equip_reason: Option<i32>,
  /// If populated, this item has a breaker type corresponding to the given value. See DestinyBreakerTypeDefinition for more details.
  #[serde(rename = "breakerType")]
  breaker_type: Option<i32>,
  /// If populated, this is the hash identifier for the item's breaker type. See DestinyBreakerTypeDefinition for more details.
  #[serde(rename = "breakerTypeHash")]
  breaker_type_hash: Option<i32>,
  /// IF populated, this item supports Energy mechanics (i.e. Armor 2.0), and these are the current details of its energy type and available capacity to spend energy points.
  #[serde(rename = "energy")]
  energy: Option<Value>
}

impl DestinyEntitiesItemsDestinyItemInstanceComponent {
  /// If an item is \"instanced\", this will contain information about the item's instance that doesn't fit easily into other components. One might say this is the \"essential\" instance data for the item.  Items are instanced if they require information or state that can vary. For instance, weapons are Instanced: they are given a unique identifier, uniquely generated stats, and can have their properties altered. Non-instanced items have none of these things: for instance, Glimmer has no unique properties aside from how much of it you own.  You can tell from an item's definition whether it will be instanced or not by looking at the DestinyInventoryItemDefinition's definition.inventory.isInstanceItem property.
  pub fn new() -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    DestinyEntitiesItemsDestinyItemInstanceComponent {
      damage_type: None,
      damage_type_hash: None,
      primary_stat: None,
      item_level: None,
      quality: None,
      is_equipped: None,
      can_equip: None,
      equip_required_level: None,
      unlock_hashes_required_to_equip: None,
      cannot_equip_reason: None,
      breaker_type: None,
      breaker_type_hash: None,
      energy: None
    }
  }

  pub fn set_damage_type(&mut self, damage_type: i32) {
    self.damage_type = Some(damage_type);
  }

  pub fn with_damage_type(mut self, damage_type: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.damage_type = Some(damage_type);
    self
  }

  pub fn damage_type(&self) -> Option<&i32> {
    self.damage_type.as_ref()
  }

  pub fn reset_damage_type(&mut self) {
    self.damage_type = None;
  }

  pub fn set_damage_type_hash(&mut self, damage_type_hash: i32) {
    self.damage_type_hash = Some(damage_type_hash);
  }

  pub fn with_damage_type_hash(mut self, damage_type_hash: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.damage_type_hash = Some(damage_type_hash);
    self
  }

  pub fn damage_type_hash(&self) -> Option<&i32> {
    self.damage_type_hash.as_ref()
  }

  pub fn reset_damage_type_hash(&mut self) {
    self.damage_type_hash = None;
  }

  pub fn set_primary_stat(&mut self, primary_stat: Value) {
    self.primary_stat = Some(primary_stat);
  }

  pub fn with_primary_stat(mut self, primary_stat: Value) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.primary_stat = Some(primary_stat);
    self
  }

  pub fn primary_stat(&self) -> Option<&Value> {
    self.primary_stat.as_ref()
  }

  pub fn reset_primary_stat(&mut self) {
    self.primary_stat = None;
  }

  pub fn set_item_level(&mut self, item_level: i32) {
    self.item_level = Some(item_level);
  }

  pub fn with_item_level(mut self, item_level: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.item_level = Some(item_level);
    self
  }

  pub fn item_level(&self) -> Option<&i32> {
    self.item_level.as_ref()
  }

  pub fn reset_item_level(&mut self) {
    self.item_level = None;
  }

  pub fn set_quality(&mut self, quality: i32) {
    self.quality = Some(quality);
  }

  pub fn with_quality(mut self, quality: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.quality = Some(quality);
    self
  }

  pub fn quality(&self) -> Option<&i32> {
    self.quality.as_ref()
  }

  pub fn reset_quality(&mut self) {
    self.quality = None;
  }

  pub fn set_is_equipped(&mut self, is_equipped: bool) {
    self.is_equipped = Some(is_equipped);
  }

  pub fn with_is_equipped(mut self, is_equipped: bool) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.is_equipped = Some(is_equipped);
    self
  }

  pub fn is_equipped(&self) -> Option<&bool> {
    self.is_equipped.as_ref()
  }

  pub fn reset_is_equipped(&mut self) {
    self.is_equipped = None;
  }

  pub fn set_can_equip(&mut self, can_equip: bool) {
    self.can_equip = Some(can_equip);
  }

  pub fn with_can_equip(mut self, can_equip: bool) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.can_equip = Some(can_equip);
    self
  }

  pub fn can_equip(&self) -> Option<&bool> {
    self.can_equip.as_ref()
  }

  pub fn reset_can_equip(&mut self) {
    self.can_equip = None;
  }

  pub fn set_equip_required_level(&mut self, equip_required_level: i32) {
    self.equip_required_level = Some(equip_required_level);
  }

  pub fn with_equip_required_level(mut self, equip_required_level: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.equip_required_level = Some(equip_required_level);
    self
  }

  pub fn equip_required_level(&self) -> Option<&i32> {
    self.equip_required_level.as_ref()
  }

  pub fn reset_equip_required_level(&mut self) {
    self.equip_required_level = None;
  }

  pub fn set_unlock_hashes_required_to_equip(&mut self, unlock_hashes_required_to_equip: Vec<i32>) {
    self.unlock_hashes_required_to_equip = Some(unlock_hashes_required_to_equip);
  }

  pub fn with_unlock_hashes_required_to_equip(mut self, unlock_hashes_required_to_equip: Vec<i32>) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.unlock_hashes_required_to_equip = Some(unlock_hashes_required_to_equip);
    self
  }

  pub fn unlock_hashes_required_to_equip(&self) -> Option<&Vec<i32>> {
    self.unlock_hashes_required_to_equip.as_ref()
  }

  pub fn reset_unlock_hashes_required_to_equip(&mut self) {
    self.unlock_hashes_required_to_equip = None;
  }

  pub fn set_cannot_equip_reason(&mut self, cannot_equip_reason: i32) {
    self.cannot_equip_reason = Some(cannot_equip_reason);
  }

  pub fn with_cannot_equip_reason(mut self, cannot_equip_reason: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.cannot_equip_reason = Some(cannot_equip_reason);
    self
  }

  pub fn cannot_equip_reason(&self) -> Option<&i32> {
    self.cannot_equip_reason.as_ref()
  }

  pub fn reset_cannot_equip_reason(&mut self) {
    self.cannot_equip_reason = None;
  }

  pub fn set_breaker_type(&mut self, breaker_type: i32) {
    self.breaker_type = Some(breaker_type);
  }

  pub fn with_breaker_type(mut self, breaker_type: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.breaker_type = Some(breaker_type);
    self
  }

  pub fn breaker_type(&self) -> Option<&i32> {
    self.breaker_type.as_ref()
  }

  pub fn reset_breaker_type(&mut self) {
    self.breaker_type = None;
  }

  pub fn set_breaker_type_hash(&mut self, breaker_type_hash: i32) {
    self.breaker_type_hash = Some(breaker_type_hash);
  }

  pub fn with_breaker_type_hash(mut self, breaker_type_hash: i32) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.breaker_type_hash = Some(breaker_type_hash);
    self
  }

  pub fn breaker_type_hash(&self) -> Option<&i32> {
    self.breaker_type_hash.as_ref()
  }

  pub fn reset_breaker_type_hash(&mut self) {
    self.breaker_type_hash = None;
  }

  pub fn set_energy(&mut self, energy: Value) {
    self.energy = Some(energy);
  }

  pub fn with_energy(mut self, energy: Value) -> DestinyEntitiesItemsDestinyItemInstanceComponent {
    self.energy = Some(energy);
    self
  }

  pub fn energy(&self) -> Option<&Value> {
    self.energy.as_ref()
  }

  pub fn reset_energy(&mut self) {
    self.energy = None;
  }

}



