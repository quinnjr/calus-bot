/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsPlugSetsDestinyPlugSetsComponent : Sockets may refer to a \"Plug Set\": a set of reusable plugs that may be shared across multiple sockets (or even, in theory, multiple sockets over multiple items).  This is the set of those plugs that we came across in the users' inventory, along with the values for plugs in the set. Any given set in this component may be represented in Character and Profile-level, as some plugs may be Profile-level restricted, and some character-level restricted. (note that the ones that are even more specific will remain on the actual socket component itself, as they cannot be reused)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsPlugSetsDestinyPlugSetsComponent {
  /// The shared list of plugs for each relevant PlugSet, keyed by the hash identifier of the PlugSet (DestinyPlugSetDefinition).
  #[serde(rename = "plugs")]
  plugs: Option<::std::collections::HashMap<String, Vec<::models::DestinySocketsDestinyItemPlug>>>
}

impl DestinyComponentsPlugSetsDestinyPlugSetsComponent {
  /// Sockets may refer to a \"Plug Set\": a set of reusable plugs that may be shared across multiple sockets (or even, in theory, multiple sockets over multiple items).  This is the set of those plugs that we came across in the users' inventory, along with the values for plugs in the set. Any given set in this component may be represented in Character and Profile-level, as some plugs may be Profile-level restricted, and some character-level restricted. (note that the ones that are even more specific will remain on the actual socket component itself, as they cannot be reused)
  pub fn new() -> DestinyComponentsPlugSetsDestinyPlugSetsComponent {
    DestinyComponentsPlugSetsDestinyPlugSetsComponent {
      plugs: None
    }
  }

  pub fn set_plugs(&mut self, plugs: ::std::collections::HashMap<String, Vec<::models::DestinySocketsDestinyItemPlug>>) {
    self.plugs = Some(plugs);
  }

  pub fn with_plugs(mut self, plugs: ::std::collections::HashMap<String, Vec<::models::DestinySocketsDestinyItemPlug>>) -> DestinyComponentsPlugSetsDestinyPlugSetsComponent {
    self.plugs = Some(plugs);
    self
  }

  pub fn plugs(&self) -> Option<&::std::collections::HashMap<String, Vec<::models::DestinySocketsDestinyItemPlug>>> {
    self.plugs.as_ref()
  }

  pub fn reset_plugs(&mut self) {
    self.plugs = None;
  }

}



