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
pub struct DestinyDefinitionsDestinyActivityLoadoutRequirement {
  #[serde(rename = "equipmentSlotHash")]
  equipment_slot_hash: Option<i32>,
  #[serde(rename = "allowedEquippedItemHashes")]
  allowed_equipped_item_hashes: Option<Vec<i32>>,
  #[serde(rename = "allowedWeaponSubTypes")]
  allowed_weapon_sub_types: Option<Vec<i32>>
}

impl DestinyDefinitionsDestinyActivityLoadoutRequirement {
  pub fn new() -> DestinyDefinitionsDestinyActivityLoadoutRequirement {
    DestinyDefinitionsDestinyActivityLoadoutRequirement {
      equipment_slot_hash: None,
      allowed_equipped_item_hashes: None,
      allowed_weapon_sub_types: None
    }
  }

  pub fn set_equipment_slot_hash(&mut self, equipment_slot_hash: i32) {
    self.equipment_slot_hash = Some(equipment_slot_hash);
  }

  pub fn with_equipment_slot_hash(mut self, equipment_slot_hash: i32) -> DestinyDefinitionsDestinyActivityLoadoutRequirement {
    self.equipment_slot_hash = Some(equipment_slot_hash);
    self
  }

  pub fn equipment_slot_hash(&self) -> Option<&i32> {
    self.equipment_slot_hash.as_ref()
  }

  pub fn reset_equipment_slot_hash(&mut self) {
    self.equipment_slot_hash = None;
  }

  pub fn set_allowed_equipped_item_hashes(&mut self, allowed_equipped_item_hashes: Vec<i32>) {
    self.allowed_equipped_item_hashes = Some(allowed_equipped_item_hashes);
  }

  pub fn with_allowed_equipped_item_hashes(mut self, allowed_equipped_item_hashes: Vec<i32>) -> DestinyDefinitionsDestinyActivityLoadoutRequirement {
    self.allowed_equipped_item_hashes = Some(allowed_equipped_item_hashes);
    self
  }

  pub fn allowed_equipped_item_hashes(&self) -> Option<&Vec<i32>> {
    self.allowed_equipped_item_hashes.as_ref()
  }

  pub fn reset_allowed_equipped_item_hashes(&mut self) {
    self.allowed_equipped_item_hashes = None;
  }

  pub fn set_allowed_weapon_sub_types(&mut self, allowed_weapon_sub_types: Vec<i32>) {
    self.allowed_weapon_sub_types = Some(allowed_weapon_sub_types);
  }

  pub fn with_allowed_weapon_sub_types(mut self, allowed_weapon_sub_types: Vec<i32>) -> DestinyDefinitionsDestinyActivityLoadoutRequirement {
    self.allowed_weapon_sub_types = Some(allowed_weapon_sub_types);
    self
  }

  pub fn allowed_weapon_sub_types(&self) -> Option<&Vec<i32>> {
    self.allowed_weapon_sub_types.as_ref()
  }

  pub fn reset_allowed_weapon_sub_types(&mut self) {
    self.allowed_weapon_sub_types = None;
  }

}


