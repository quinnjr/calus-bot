/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition : The definition for an \"inventory flyout\": a UI screen where we show you part of an otherwise hidden vendor inventory: like the Vault inventory buckets.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
  /// If the flyout is locked, this is the reason why.
  #[serde(rename = "lockedDescription")]
  locked_description: Option<String>,
  /// The title and other common properties of the flyout.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  /// A list of inventory buckets and other metadata to show on the screen.
  #[serde(rename = "buckets")]
  buckets: Option<Vec<::models::DestinyDefinitionsDestinyVendorInventoryFlyoutBucketDefinition>>,
  /// An identifier for the flyout, in case anything else needs to refer to them.
  #[serde(rename = "flyoutId")]
  flyout_id: Option<i32>,
  /// If this is true, don't show any of the glistening \"this is a new item\" UI elements, like we show on the inventory items themselves in in-game UI.
  #[serde(rename = "suppressNewness")]
  suppress_newness: Option<bool>,
  /// If this flyout is meant to show you the contents of the player's equipment slot, this is the slot to show.
  #[serde(rename = "equipmentSlotHash")]
  equipment_slot_hash: Option<i32>
}

impl DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
  /// The definition for an \"inventory flyout\": a UI screen where we show you part of an otherwise hidden vendor inventory: like the Vault inventory buckets.
  pub fn new() -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
      locked_description: None,
      display_properties: None,
      buckets: None,
      flyout_id: None,
      suppress_newness: None,
      equipment_slot_hash: None
    }
  }

  pub fn set_locked_description(&mut self, locked_description: String) {
    self.locked_description = Some(locked_description);
  }

  pub fn with_locked_description(mut self, locked_description: String) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.locked_description = Some(locked_description);
    self
  }

  pub fn locked_description(&self) -> Option<&String> {
    self.locked_description.as_ref()
  }

  pub fn reset_locked_description(&mut self) {
    self.locked_description = None;
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_buckets(&mut self, buckets: Vec<::models::DestinyDefinitionsDestinyVendorInventoryFlyoutBucketDefinition>) {
    self.buckets = Some(buckets);
  }

  pub fn with_buckets(mut self, buckets: Vec<::models::DestinyDefinitionsDestinyVendorInventoryFlyoutBucketDefinition>) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.buckets = Some(buckets);
    self
  }

  pub fn buckets(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyVendorInventoryFlyoutBucketDefinition>> {
    self.buckets.as_ref()
  }

  pub fn reset_buckets(&mut self) {
    self.buckets = None;
  }

  pub fn set_flyout_id(&mut self, flyout_id: i32) {
    self.flyout_id = Some(flyout_id);
  }

  pub fn with_flyout_id(mut self, flyout_id: i32) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.flyout_id = Some(flyout_id);
    self
  }

  pub fn flyout_id(&self) -> Option<&i32> {
    self.flyout_id.as_ref()
  }

  pub fn reset_flyout_id(&mut self) {
    self.flyout_id = None;
  }

  pub fn set_suppress_newness(&mut self, suppress_newness: bool) {
    self.suppress_newness = Some(suppress_newness);
  }

  pub fn with_suppress_newness(mut self, suppress_newness: bool) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.suppress_newness = Some(suppress_newness);
    self
  }

  pub fn suppress_newness(&self) -> Option<&bool> {
    self.suppress_newness.as_ref()
  }

  pub fn reset_suppress_newness(&mut self) {
    self.suppress_newness = None;
  }

  pub fn set_equipment_slot_hash(&mut self, equipment_slot_hash: i32) {
    self.equipment_slot_hash = Some(equipment_slot_hash);
  }

  pub fn with_equipment_slot_hash(mut self, equipment_slot_hash: i32) -> DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition {
    self.equipment_slot_hash = Some(equipment_slot_hash);
    self
  }

  pub fn equipment_slot_hash(&self) -> Option<&i32> {
    self.equipment_slot_hash.as_ref()
  }

  pub fn reset_equipment_slot_hash(&mut self) {
    self.equipment_slot_hash = None;
  }

}


