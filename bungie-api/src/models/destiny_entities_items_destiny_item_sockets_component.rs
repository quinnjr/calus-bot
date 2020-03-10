/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesItemsDestinyItemSocketsComponent : Instanced items can have sockets, which are slots on the item where plugs can be inserted.  Sockets are a bit complex: be sure to examine the documentation on the DestinyInventoryItemDefinition's \"socket\" block and elsewhere on these objects for more details.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesItemsDestinyItemSocketsComponent {
  /// The list of all sockets on the item, and their status information.
  #[serde(rename = "sockets")]
  sockets: Option<Vec<::models::DestinyEntitiesItemsDestinyItemSocketState>>
}

impl DestinyEntitiesItemsDestinyItemSocketsComponent {
  /// Instanced items can have sockets, which are slots on the item where plugs can be inserted.  Sockets are a bit complex: be sure to examine the documentation on the DestinyInventoryItemDefinition's \"socket\" block and elsewhere on these objects for more details.
  pub fn new() -> DestinyEntitiesItemsDestinyItemSocketsComponent {
    DestinyEntitiesItemsDestinyItemSocketsComponent {
      sockets: None
    }
  }

  pub fn set_sockets(&mut self, sockets: Vec<::models::DestinyEntitiesItemsDestinyItemSocketState>) {
    self.sockets = Some(sockets);
  }

  pub fn with_sockets(mut self, sockets: Vec<::models::DestinyEntitiesItemsDestinyItemSocketState>) -> DestinyEntitiesItemsDestinyItemSocketsComponent {
    self.sockets = Some(sockets);
    self
  }

  pub fn sockets(&self) -> Option<&Vec<::models::DestinyEntitiesItemsDestinyItemSocketState>> {
    self.sockets.as_ref()
  }

  pub fn reset_sockets(&mut self) {
    self.sockets = None;
  }

}


