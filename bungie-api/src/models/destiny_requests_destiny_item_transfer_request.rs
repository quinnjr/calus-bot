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
pub struct DestinyRequestsDestinyItemTransferRequest {
  #[serde(rename = "itemReferenceHash")]
  item_reference_hash: Option<i32>,
  #[serde(rename = "stackSize")]
  stack_size: Option<i32>,
  #[serde(rename = "transferToVault")]
  transfer_to_vault: Option<bool>,
  #[serde(rename = "itemId")]
  item_id: Option<i64>,
  #[serde(rename = "characterId")]
  character_id: Option<i64>,
  #[serde(rename = "membershipType")]
  membership_type: Option<i32>
}

impl DestinyRequestsDestinyItemTransferRequest {
  pub fn new() -> DestinyRequestsDestinyItemTransferRequest {
    DestinyRequestsDestinyItemTransferRequest {
      item_reference_hash: None,
      stack_size: None,
      transfer_to_vault: None,
      item_id: None,
      character_id: None,
      membership_type: None
    }
  }

  pub fn set_item_reference_hash(&mut self, item_reference_hash: i32) {
    self.item_reference_hash = Some(item_reference_hash);
  }

  pub fn with_item_reference_hash(mut self, item_reference_hash: i32) -> DestinyRequestsDestinyItemTransferRequest {
    self.item_reference_hash = Some(item_reference_hash);
    self
  }

  pub fn item_reference_hash(&self) -> Option<&i32> {
    self.item_reference_hash.as_ref()
  }

  pub fn reset_item_reference_hash(&mut self) {
    self.item_reference_hash = None;
  }

  pub fn set_stack_size(&mut self, stack_size: i32) {
    self.stack_size = Some(stack_size);
  }

  pub fn with_stack_size(mut self, stack_size: i32) -> DestinyRequestsDestinyItemTransferRequest {
    self.stack_size = Some(stack_size);
    self
  }

  pub fn stack_size(&self) -> Option<&i32> {
    self.stack_size.as_ref()
  }

  pub fn reset_stack_size(&mut self) {
    self.stack_size = None;
  }

  pub fn set_transfer_to_vault(&mut self, transfer_to_vault: bool) {
    self.transfer_to_vault = Some(transfer_to_vault);
  }

  pub fn with_transfer_to_vault(mut self, transfer_to_vault: bool) -> DestinyRequestsDestinyItemTransferRequest {
    self.transfer_to_vault = Some(transfer_to_vault);
    self
  }

  pub fn transfer_to_vault(&self) -> Option<&bool> {
    self.transfer_to_vault.as_ref()
  }

  pub fn reset_transfer_to_vault(&mut self) {
    self.transfer_to_vault = None;
  }

  pub fn set_item_id(&mut self, item_id: i64) {
    self.item_id = Some(item_id);
  }

  pub fn with_item_id(mut self, item_id: i64) -> DestinyRequestsDestinyItemTransferRequest {
    self.item_id = Some(item_id);
    self
  }

  pub fn item_id(&self) -> Option<&i64> {
    self.item_id.as_ref()
  }

  pub fn reset_item_id(&mut self) {
    self.item_id = None;
  }

  pub fn set_character_id(&mut self, character_id: i64) {
    self.character_id = Some(character_id);
  }

  pub fn with_character_id(mut self, character_id: i64) -> DestinyRequestsDestinyItemTransferRequest {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i64> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

  pub fn set_membership_type(&mut self, membership_type: i32) {
    self.membership_type = Some(membership_type);
  }

  pub fn with_membership_type(mut self, membership_type: i32) -> DestinyRequestsDestinyItemTransferRequest {
    self.membership_type = Some(membership_type);
    self
  }

  pub fn membership_type(&self) -> Option<&i32> {
    self.membership_type.as_ref()
  }

  pub fn reset_membership_type(&mut self) {
    self.membership_type = None;
  }

}


