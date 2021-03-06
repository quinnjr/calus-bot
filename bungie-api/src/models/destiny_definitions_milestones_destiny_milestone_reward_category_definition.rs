/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition : The definition of a category of rewards, that contains many individual rewards.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
  /// Identifies the reward category. Only guaranteed unique within this specific component!
  #[serde(rename = "categoryHash")]
  category_hash: Option<i32>,
  /// The string identifier for the category, if you want to use it for some end. Guaranteed unique within the specific component.
  #[serde(rename = "categoryIdentifier")]
  category_identifier: Option<String>,
  /// Hopefully this is obvious by now.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  /// If this milestone can provide rewards, this will define the sets of rewards that can be earned, the conditions under which they can be acquired, internal data that we'll use at runtime to determine whether you've already earned or redeemed this set of rewards, and the category that this reward should be placed under.
  #[serde(rename = "rewardEntries")]
  reward_entries: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinition>>,
  /// If you want to use BNet's recommended order for rendering categories programmatically, use this value and compare it to other categories to determine the order in which they should be rendered. I don't feel great about putting this here, I won't lie.
  #[serde(rename = "order")]
  order: Option<i32>
}

impl DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
  /// The definition of a category of rewards, that contains many individual rewards.
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
      category_hash: None,
      category_identifier: None,
      display_properties: None,
      reward_entries: None,
      order: None
    }
  }

  pub fn set_category_hash(&mut self, category_hash: i32) {
    self.category_hash = Some(category_hash);
  }

  pub fn with_category_hash(mut self, category_hash: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    self.category_hash = Some(category_hash);
    self
  }

  pub fn category_hash(&self) -> Option<&i32> {
    self.category_hash.as_ref()
  }

  pub fn reset_category_hash(&mut self) {
    self.category_hash = None;
  }

  pub fn set_category_identifier(&mut self, category_identifier: String) {
    self.category_identifier = Some(category_identifier);
  }

  pub fn with_category_identifier(mut self, category_identifier: String) -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    self.category_identifier = Some(category_identifier);
    self
  }

  pub fn category_identifier(&self) -> Option<&String> {
    self.category_identifier.as_ref()
  }

  pub fn reset_category_identifier(&mut self) {
    self.category_identifier = None;
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_reward_entries(&mut self, reward_entries: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinition>) {
    self.reward_entries = Some(reward_entries);
  }

  pub fn with_reward_entries(mut self, reward_entries: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    self.reward_entries = Some(reward_entries);
    self
  }

  pub fn reward_entries(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinition>> {
    self.reward_entries.as_ref()
  }

  pub fn reset_reward_entries(&mut self) {
    self.reward_entries = None;
  }

  pub fn set_order(&mut self, order: i32) {
    self.order = Some(order);
  }

  pub fn with_order(mut self, order: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition {
    self.order = Some(order);
    self
  }

  pub fn order(&self) -> Option<&i32> {
    self.order.as_ref()
  }

  pub fn reset_order(&mut self) {
    self.order = None;
  }

}



