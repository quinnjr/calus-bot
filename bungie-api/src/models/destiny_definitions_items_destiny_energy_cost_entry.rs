/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsItemsDestinyEnergyCostEntry : Some plugs cost Energy, which is a stat on the item that can be increased by other plugs (that, at least in Armor 2.0, have a \"masterworks-like\" mechanic for upgrading). If a plug has costs, the details of that cost are defined here.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyEnergyCostEntry {
  /// The Energy cost for inserting this plug.
  #[serde(rename = "energyCost")]
  energy_cost: Option<i32>,
  /// The type of energy that this plug costs, as a reference to the DestinyEnergyTypeDefinition of the energy type.
  #[serde(rename = "energyTypeHash")]
  energy_type_hash: Option<i32>,
  /// The type of energy that this plug costs, in enum form.
  #[serde(rename = "energyType")]
  energy_type: Option<i32>
}

impl DestinyDefinitionsItemsDestinyEnergyCostEntry {
  /// Some plugs cost Energy, which is a stat on the item that can be increased by other plugs (that, at least in Armor 2.0, have a \"masterworks-like\" mechanic for upgrading). If a plug has costs, the details of that cost are defined here.
  pub fn new() -> DestinyDefinitionsItemsDestinyEnergyCostEntry {
    DestinyDefinitionsItemsDestinyEnergyCostEntry {
      energy_cost: None,
      energy_type_hash: None,
      energy_type: None
    }
  }

  pub fn set_energy_cost(&mut self, energy_cost: i32) {
    self.energy_cost = Some(energy_cost);
  }

  pub fn with_energy_cost(mut self, energy_cost: i32) -> DestinyDefinitionsItemsDestinyEnergyCostEntry {
    self.energy_cost = Some(energy_cost);
    self
  }

  pub fn energy_cost(&self) -> Option<&i32> {
    self.energy_cost.as_ref()
  }

  pub fn reset_energy_cost(&mut self) {
    self.energy_cost = None;
  }

  pub fn set_energy_type_hash(&mut self, energy_type_hash: i32) {
    self.energy_type_hash = Some(energy_type_hash);
  }

  pub fn with_energy_type_hash(mut self, energy_type_hash: i32) -> DestinyDefinitionsItemsDestinyEnergyCostEntry {
    self.energy_type_hash = Some(energy_type_hash);
    self
  }

  pub fn energy_type_hash(&self) -> Option<&i32> {
    self.energy_type_hash.as_ref()
  }

  pub fn reset_energy_type_hash(&mut self) {
    self.energy_type_hash = None;
  }

  pub fn set_energy_type(&mut self, energy_type: i32) {
    self.energy_type = Some(energy_type);
  }

  pub fn with_energy_type(mut self, energy_type: i32) -> DestinyDefinitionsItemsDestinyEnergyCostEntry {
    self.energy_type = Some(energy_type);
    self
  }

  pub fn energy_type(&self) -> Option<&i32> {
    self.energy_type.as_ref()
  }

  pub fn reset_energy_type(&mut self) {
    self.energy_type = None;
  }

}



