/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyVendorItemSocketOverride : The information for how the vendor purchase should override a given socket with custom plug data.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyVendorItemSocketOverride {
  /// If this is populated, the socket will be overridden with a specific plug.  If this isn't populated, it's being overridden by something more complicated that is only known by the Game Server and God, which means we can't tell you in advance what it'll be.
  #[serde(rename = "singleItemHash")]
  single_item_hash: Option<i32>,
  /// If this is greater than -1, the number of randomized plugs on this socket will be set to this quantity instead of whatever it's set to by default.
  #[serde(rename = "randomizedOptionsCount")]
  randomized_options_count: Option<i32>,
  /// This appears to be used to select which socket ultimately gets the override defined here.
  #[serde(rename = "socketTypeHash")]
  socket_type_hash: Option<i32>
}

impl DestinyDefinitionsDestinyVendorItemSocketOverride {
  /// The information for how the vendor purchase should override a given socket with custom plug data.
  pub fn new() -> DestinyDefinitionsDestinyVendorItemSocketOverride {
    DestinyDefinitionsDestinyVendorItemSocketOverride {
      single_item_hash: None,
      randomized_options_count: None,
      socket_type_hash: None
    }
  }

  pub fn set_single_item_hash(&mut self, single_item_hash: i32) {
    self.single_item_hash = Some(single_item_hash);
  }

  pub fn with_single_item_hash(mut self, single_item_hash: i32) -> DestinyDefinitionsDestinyVendorItemSocketOverride {
    self.single_item_hash = Some(single_item_hash);
    self
  }

  pub fn single_item_hash(&self) -> Option<&i32> {
    self.single_item_hash.as_ref()
  }

  pub fn reset_single_item_hash(&mut self) {
    self.single_item_hash = None;
  }

  pub fn set_randomized_options_count(&mut self, randomized_options_count: i32) {
    self.randomized_options_count = Some(randomized_options_count);
  }

  pub fn with_randomized_options_count(mut self, randomized_options_count: i32) -> DestinyDefinitionsDestinyVendorItemSocketOverride {
    self.randomized_options_count = Some(randomized_options_count);
    self
  }

  pub fn randomized_options_count(&self) -> Option<&i32> {
    self.randomized_options_count.as_ref()
  }

  pub fn reset_randomized_options_count(&mut self) {
    self.randomized_options_count = None;
  }

  pub fn set_socket_type_hash(&mut self, socket_type_hash: i32) {
    self.socket_type_hash = Some(socket_type_hash);
  }

  pub fn with_socket_type_hash(mut self, socket_type_hash: i32) -> DestinyDefinitionsDestinyVendorItemSocketOverride {
    self.socket_type_hash = Some(socket_type_hash);
    self
  }

  pub fn socket_type_hash(&self) -> Option<&i32> {
    self.socket_type_hash.as_ref()
  }

  pub fn reset_socket_type_hash(&mut self) {
    self.socket_type_hash = None;
  }

}



