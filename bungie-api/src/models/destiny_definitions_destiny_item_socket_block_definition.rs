/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemSocketBlockDefinition : If defined, the item has at least one socket.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemSocketBlockDefinition {
  /// This was supposed to be a string that would give per-item details about sockets. In practice, it turns out that all this ever has is the localized word \"details\". ... that's lame, but perhaps it will become something cool in the future.
  #[serde(rename = "detail")]
  detail: Option<String>,
  /// Each non-intrinsic (or mutable) socket on an item is defined here. Check inside for more info.
  #[serde(rename = "socketEntries")]
  socket_entries: Option<Vec<::models::DestinyDefinitionsDestinyItemSocketEntryDefinition>>,
  /// Each intrinsic (or immutable/permanent) socket on an item is defined here, along with the plug that is permanently affixed to the socket.
  #[serde(rename = "intrinsicSockets")]
  intrinsic_sockets: Option<Vec<::models::DestinyDefinitionsDestinyItemIntrinsicSocketEntryDefinition>>,
  /// A convenience property, that refers to the sockets in the \"sockets\" property, pre-grouped by category and ordered in the manner that they should be grouped in the UI. You could form this yourself with the existing data, but why would you want to? Enjoy life man.
  #[serde(rename = "socketCategories")]
  socket_categories: Option<Vec<::models::DestinyDefinitionsDestinyItemSocketCategoryDefinition>>
}

impl DestinyDefinitionsDestinyItemSocketBlockDefinition {
  /// If defined, the item has at least one socket.
  pub fn new() -> DestinyDefinitionsDestinyItemSocketBlockDefinition {
    DestinyDefinitionsDestinyItemSocketBlockDefinition {
      detail: None,
      socket_entries: None,
      intrinsic_sockets: None,
      socket_categories: None
    }
  }

  pub fn set_detail(&mut self, detail: String) {
    self.detail = Some(detail);
  }

  pub fn with_detail(mut self, detail: String) -> DestinyDefinitionsDestinyItemSocketBlockDefinition {
    self.detail = Some(detail);
    self
  }

  pub fn detail(&self) -> Option<&String> {
    self.detail.as_ref()
  }

  pub fn reset_detail(&mut self) {
    self.detail = None;
  }

  pub fn set_socket_entries(&mut self, socket_entries: Vec<::models::DestinyDefinitionsDestinyItemSocketEntryDefinition>) {
    self.socket_entries = Some(socket_entries);
  }

  pub fn with_socket_entries(mut self, socket_entries: Vec<::models::DestinyDefinitionsDestinyItemSocketEntryDefinition>) -> DestinyDefinitionsDestinyItemSocketBlockDefinition {
    self.socket_entries = Some(socket_entries);
    self
  }

  pub fn socket_entries(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyItemSocketEntryDefinition>> {
    self.socket_entries.as_ref()
  }

  pub fn reset_socket_entries(&mut self) {
    self.socket_entries = None;
  }

  pub fn set_intrinsic_sockets(&mut self, intrinsic_sockets: Vec<::models::DestinyDefinitionsDestinyItemIntrinsicSocketEntryDefinition>) {
    self.intrinsic_sockets = Some(intrinsic_sockets);
  }

  pub fn with_intrinsic_sockets(mut self, intrinsic_sockets: Vec<::models::DestinyDefinitionsDestinyItemIntrinsicSocketEntryDefinition>) -> DestinyDefinitionsDestinyItemSocketBlockDefinition {
    self.intrinsic_sockets = Some(intrinsic_sockets);
    self
  }

  pub fn intrinsic_sockets(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyItemIntrinsicSocketEntryDefinition>> {
    self.intrinsic_sockets.as_ref()
  }

  pub fn reset_intrinsic_sockets(&mut self) {
    self.intrinsic_sockets = None;
  }

  pub fn set_socket_categories(&mut self, socket_categories: Vec<::models::DestinyDefinitionsDestinyItemSocketCategoryDefinition>) {
    self.socket_categories = Some(socket_categories);
  }

  pub fn with_socket_categories(mut self, socket_categories: Vec<::models::DestinyDefinitionsDestinyItemSocketCategoryDefinition>) -> DestinyDefinitionsDestinyItemSocketBlockDefinition {
    self.socket_categories = Some(socket_categories);
    self
  }

  pub fn socket_categories(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyItemSocketCategoryDefinition>> {
    self.socket_categories.as_ref()
  }

  pub fn reset_socket_categories(&mut self) {
    self.socket_categories = None;
  }

}



