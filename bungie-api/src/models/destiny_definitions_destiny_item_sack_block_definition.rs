/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemSackBlockDefinition : Some items are \"sacks\" - they can be \"opened\" to produce other items. This is information related to its sack status, mostly UI strings. Engrams are an example of items that are considered to be \"Sacks\".

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemSackBlockDefinition {
  /// A description of what will happen when you open the sack. As far as I can tell, this is blank currently. Unknown whether it will eventually be populated with useful info.
  #[serde(rename = "detailAction")]
  detail_action: Option<String>,
  /// The localized name of the action being performed when you open the sack.
  #[serde(rename = "openAction")]
  open_action: Option<String>,
  #[serde(rename = "selectItemCount")]
  select_item_count: Option<i32>,
  #[serde(rename = "vendorSackType")]
  vendor_sack_type: Option<String>,
  #[serde(rename = "openOnAcquire")]
  open_on_acquire: Option<bool>
}

impl DestinyDefinitionsDestinyItemSackBlockDefinition {
  /// Some items are \"sacks\" - they can be \"opened\" to produce other items. This is information related to its sack status, mostly UI strings. Engrams are an example of items that are considered to be \"Sacks\".
  pub fn new() -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    DestinyDefinitionsDestinyItemSackBlockDefinition {
      detail_action: None,
      open_action: None,
      select_item_count: None,
      vendor_sack_type: None,
      open_on_acquire: None
    }
  }

  pub fn set_detail_action(&mut self, detail_action: String) {
    self.detail_action = Some(detail_action);
  }

  pub fn with_detail_action(mut self, detail_action: String) -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    self.detail_action = Some(detail_action);
    self
  }

  pub fn detail_action(&self) -> Option<&String> {
    self.detail_action.as_ref()
  }

  pub fn reset_detail_action(&mut self) {
    self.detail_action = None;
  }

  pub fn set_open_action(&mut self, open_action: String) {
    self.open_action = Some(open_action);
  }

  pub fn with_open_action(mut self, open_action: String) -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    self.open_action = Some(open_action);
    self
  }

  pub fn open_action(&self) -> Option<&String> {
    self.open_action.as_ref()
  }

  pub fn reset_open_action(&mut self) {
    self.open_action = None;
  }

  pub fn set_select_item_count(&mut self, select_item_count: i32) {
    self.select_item_count = Some(select_item_count);
  }

  pub fn with_select_item_count(mut self, select_item_count: i32) -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    self.select_item_count = Some(select_item_count);
    self
  }

  pub fn select_item_count(&self) -> Option<&i32> {
    self.select_item_count.as_ref()
  }

  pub fn reset_select_item_count(&mut self) {
    self.select_item_count = None;
  }

  pub fn set_vendor_sack_type(&mut self, vendor_sack_type: String) {
    self.vendor_sack_type = Some(vendor_sack_type);
  }

  pub fn with_vendor_sack_type(mut self, vendor_sack_type: String) -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    self.vendor_sack_type = Some(vendor_sack_type);
    self
  }

  pub fn vendor_sack_type(&self) -> Option<&String> {
    self.vendor_sack_type.as_ref()
  }

  pub fn reset_vendor_sack_type(&mut self) {
    self.vendor_sack_type = None;
  }

  pub fn set_open_on_acquire(&mut self, open_on_acquire: bool) {
    self.open_on_acquire = Some(open_on_acquire);
  }

  pub fn with_open_on_acquire(mut self, open_on_acquire: bool) -> DestinyDefinitionsDestinyItemSackBlockDefinition {
    self.open_on_acquire = Some(open_on_acquire);
    self
  }

  pub fn open_on_acquire(&self) -> Option<&bool> {
    self.open_on_acquire.as_ref()
  }

  pub fn reset_open_on_acquire(&mut self) {
    self.open_on_acquire = None;
  }

}


