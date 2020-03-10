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
pub struct DestinyDefinitionsDestinyProgressionRewardItemQuantity {
  #[serde(rename = "rewardedAtProgressionLevel")]
  rewarded_at_progression_level: Option<i32>,
  #[serde(rename = "acquisitionBehavior")]
  acquisition_behavior: Option<i32>,
  #[serde(rename = "uiDisplayStyle")]
  ui_display_style: Option<String>,
  #[serde(rename = "claimUnlockDisplayStrings")]
  claim_unlock_display_strings: Option<Vec<String>>,
  /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
  #[serde(rename = "itemInstanceId")]
  item_instance_id: Option<i64>,
  /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
  #[serde(rename = "quantity")]
  quantity: Option<i32>
}

impl DestinyDefinitionsDestinyProgressionRewardItemQuantity {
  pub fn new() -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    DestinyDefinitionsDestinyProgressionRewardItemQuantity {
      rewarded_at_progression_level: None,
      acquisition_behavior: None,
      ui_display_style: None,
      claim_unlock_display_strings: None,
      item_hash: None,
      item_instance_id: None,
      quantity: None
    }
  }

  pub fn set_rewarded_at_progression_level(&mut self, rewarded_at_progression_level: i32) {
    self.rewarded_at_progression_level = Some(rewarded_at_progression_level);
  }

  pub fn with_rewarded_at_progression_level(mut self, rewarded_at_progression_level: i32) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.rewarded_at_progression_level = Some(rewarded_at_progression_level);
    self
  }

  pub fn rewarded_at_progression_level(&self) -> Option<&i32> {
    self.rewarded_at_progression_level.as_ref()
  }

  pub fn reset_rewarded_at_progression_level(&mut self) {
    self.rewarded_at_progression_level = None;
  }

  pub fn set_acquisition_behavior(&mut self, acquisition_behavior: i32) {
    self.acquisition_behavior = Some(acquisition_behavior);
  }

  pub fn with_acquisition_behavior(mut self, acquisition_behavior: i32) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.acquisition_behavior = Some(acquisition_behavior);
    self
  }

  pub fn acquisition_behavior(&self) -> Option<&i32> {
    self.acquisition_behavior.as_ref()
  }

  pub fn reset_acquisition_behavior(&mut self) {
    self.acquisition_behavior = None;
  }

  pub fn set_ui_display_style(&mut self, ui_display_style: String) {
    self.ui_display_style = Some(ui_display_style);
  }

  pub fn with_ui_display_style(mut self, ui_display_style: String) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.ui_display_style = Some(ui_display_style);
    self
  }

  pub fn ui_display_style(&self) -> Option<&String> {
    self.ui_display_style.as_ref()
  }

  pub fn reset_ui_display_style(&mut self) {
    self.ui_display_style = None;
  }

  pub fn set_claim_unlock_display_strings(&mut self, claim_unlock_display_strings: Vec<String>) {
    self.claim_unlock_display_strings = Some(claim_unlock_display_strings);
  }

  pub fn with_claim_unlock_display_strings(mut self, claim_unlock_display_strings: Vec<String>) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.claim_unlock_display_strings = Some(claim_unlock_display_strings);
    self
  }

  pub fn claim_unlock_display_strings(&self) -> Option<&Vec<String>> {
    self.claim_unlock_display_strings.as_ref()
  }

  pub fn reset_claim_unlock_display_strings(&mut self) {
    self.claim_unlock_display_strings = None;
  }

  pub fn set_item_hash(&mut self, item_hash: i32) {
    self.item_hash = Some(item_hash);
  }

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_item_instance_id(&mut self, item_instance_id: i64) {
    self.item_instance_id = Some(item_instance_id);
  }

  pub fn with_item_instance_id(mut self, item_instance_id: i64) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.item_instance_id = Some(item_instance_id);
    self
  }

  pub fn item_instance_id(&self) -> Option<&i64> {
    self.item_instance_id.as_ref()
  }

  pub fn reset_item_instance_id(&mut self) {
    self.item_instance_id = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = Some(quantity);
  }

  pub fn with_quantity(mut self, quantity: i32) -> DestinyDefinitionsDestinyProgressionRewardItemQuantity {
    self.quantity = Some(quantity);
    self
  }

  pub fn quantity(&self) -> Option<&i32> {
    self.quantity.as_ref()
  }

  pub fn reset_quantity(&mut self) {
    self.quantity = None;
  }

}



