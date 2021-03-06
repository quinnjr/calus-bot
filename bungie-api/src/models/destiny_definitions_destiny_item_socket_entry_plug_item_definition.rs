/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition : The definition of a known, reusable plug that can be applied to a socket.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition {
  /// The hash identifier of a DestinyInventoryItemDefinition representing the plug that can be inserted.
  #[serde(rename = "plugItemHash")]
  plug_item_hash: Option<i32>
}

impl DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition {
  /// The definition of a known, reusable plug that can be applied to a socket.
  pub fn new() -> DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition {
    DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition {
      plug_item_hash: None
    }
  }

  pub fn set_plug_item_hash(&mut self, plug_item_hash: i32) {
    self.plug_item_hash = Some(plug_item_hash);
  }

  pub fn with_plug_item_hash(mut self, plug_item_hash: i32) -> DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition {
    self.plug_item_hash = Some(plug_item_hash);
    self
  }

  pub fn plug_item_hash(&self) -> Option<&i32> {
    self.plug_item_hash.as_ref()
  }

  pub fn reset_plug_item_hash(&mut self) {
    self.plug_item_hash = None;
  }

}



