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
pub struct DestinyDefinitionsRecordsDestinyRecordDefinition {
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// Indicates whether this Record's state is determined on a per-character or on an account-wide basis.
  #[serde(rename = "scope")]
  scope: Option<i32>,
  #[serde(rename = "presentationInfo")]
  presentation_info: Option<::models::DestinyDefinitionsPresentationDestinyPresentationChildBlock>,
  #[serde(rename = "loreHash")]
  lore_hash: Option<i32>,
  #[serde(rename = "objectiveHashes")]
  objective_hashes: Option<Vec<i32>>,
  #[serde(rename = "recordValueStyle")]
  record_value_style: Option<i32>,
  #[serde(rename = "titleInfo")]
  title_info: Option<::models::DestinyDefinitionsRecordsDestinyRecordTitleBlock>,
  #[serde(rename = "completionInfo")]
  completion_info: Option<::models::DestinyDefinitionsRecordsDestinyRecordCompletionBlock>,
  #[serde(rename = "stateInfo")]
  state_info: Option<::models::DestinyDefinitionsRecordsSchemaRecordStateBlock>,
  #[serde(rename = "requirements")]
  requirements: Option<::models::DestinyDefinitionsPresentationDestinyPresentationNodeRequirementsBlock>,
  #[serde(rename = "expirationInfo")]
  expiration_info: Option<::models::DestinyDefinitionsRecordsDestinyRecordExpirationBlock>,
  /// Some records have multiple 'interval' objectives, and the record may be claimed at each completed interval
  #[serde(rename = "intervalInfo")]
  interval_info: Option<Value>,
  /// If there is any publicly available information about rewards earned for achieving this record, this is the list of those items.   However, note that some records intentionally have \"hidden\" rewards. These will not be returned in this list.
  #[serde(rename = "rewardItems")]
  reward_items: Option<Vec<::models::DestinyDestinyItemQuantity>>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsRecordsDestinyRecordDefinition {
  pub fn new() -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    DestinyDefinitionsRecordsDestinyRecordDefinition {
      display_properties: None,
      scope: None,
      presentation_info: None,
      lore_hash: None,
      objective_hashes: None,
      record_value_style: None,
      title_info: None,
      completion_info: None,
      state_info: None,
      requirements: None,
      expiration_info: None,
      interval_info: None,
      reward_items: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_scope(&mut self, scope: i32) {
    self.scope = Some(scope);
  }

  pub fn with_scope(mut self, scope: i32) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.scope = Some(scope);
    self
  }

  pub fn scope(&self) -> Option<&i32> {
    self.scope.as_ref()
  }

  pub fn reset_scope(&mut self) {
    self.scope = None;
  }

  pub fn set_presentation_info(&mut self, presentation_info: ::models::DestinyDefinitionsPresentationDestinyPresentationChildBlock) {
    self.presentation_info = Some(presentation_info);
  }

  pub fn with_presentation_info(mut self, presentation_info: ::models::DestinyDefinitionsPresentationDestinyPresentationChildBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.presentation_info = Some(presentation_info);
    self
  }

  pub fn presentation_info(&self) -> Option<&::models::DestinyDefinitionsPresentationDestinyPresentationChildBlock> {
    self.presentation_info.as_ref()
  }

  pub fn reset_presentation_info(&mut self) {
    self.presentation_info = None;
  }

  pub fn set_lore_hash(&mut self, lore_hash: i32) {
    self.lore_hash = Some(lore_hash);
  }

  pub fn with_lore_hash(mut self, lore_hash: i32) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.lore_hash = Some(lore_hash);
    self
  }

  pub fn lore_hash(&self) -> Option<&i32> {
    self.lore_hash.as_ref()
  }

  pub fn reset_lore_hash(&mut self) {
    self.lore_hash = None;
  }

  pub fn set_objective_hashes(&mut self, objective_hashes: Vec<i32>) {
    self.objective_hashes = Some(objective_hashes);
  }

  pub fn with_objective_hashes(mut self, objective_hashes: Vec<i32>) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.objective_hashes = Some(objective_hashes);
    self
  }

  pub fn objective_hashes(&self) -> Option<&Vec<i32>> {
    self.objective_hashes.as_ref()
  }

  pub fn reset_objective_hashes(&mut self) {
    self.objective_hashes = None;
  }

  pub fn set_record_value_style(&mut self, record_value_style: i32) {
    self.record_value_style = Some(record_value_style);
  }

  pub fn with_record_value_style(mut self, record_value_style: i32) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.record_value_style = Some(record_value_style);
    self
  }

  pub fn record_value_style(&self) -> Option<&i32> {
    self.record_value_style.as_ref()
  }

  pub fn reset_record_value_style(&mut self) {
    self.record_value_style = None;
  }

  pub fn set_title_info(&mut self, title_info: ::models::DestinyDefinitionsRecordsDestinyRecordTitleBlock) {
    self.title_info = Some(title_info);
  }

  pub fn with_title_info(mut self, title_info: ::models::DestinyDefinitionsRecordsDestinyRecordTitleBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.title_info = Some(title_info);
    self
  }

  pub fn title_info(&self) -> Option<&::models::DestinyDefinitionsRecordsDestinyRecordTitleBlock> {
    self.title_info.as_ref()
  }

  pub fn reset_title_info(&mut self) {
    self.title_info = None;
  }

  pub fn set_completion_info(&mut self, completion_info: ::models::DestinyDefinitionsRecordsDestinyRecordCompletionBlock) {
    self.completion_info = Some(completion_info);
  }

  pub fn with_completion_info(mut self, completion_info: ::models::DestinyDefinitionsRecordsDestinyRecordCompletionBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.completion_info = Some(completion_info);
    self
  }

  pub fn completion_info(&self) -> Option<&::models::DestinyDefinitionsRecordsDestinyRecordCompletionBlock> {
    self.completion_info.as_ref()
  }

  pub fn reset_completion_info(&mut self) {
    self.completion_info = None;
  }

  pub fn set_state_info(&mut self, state_info: ::models::DestinyDefinitionsRecordsSchemaRecordStateBlock) {
    self.state_info = Some(state_info);
  }

  pub fn with_state_info(mut self, state_info: ::models::DestinyDefinitionsRecordsSchemaRecordStateBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.state_info = Some(state_info);
    self
  }

  pub fn state_info(&self) -> Option<&::models::DestinyDefinitionsRecordsSchemaRecordStateBlock> {
    self.state_info.as_ref()
  }

  pub fn reset_state_info(&mut self) {
    self.state_info = None;
  }

  pub fn set_requirements(&mut self, requirements: ::models::DestinyDefinitionsPresentationDestinyPresentationNodeRequirementsBlock) {
    self.requirements = Some(requirements);
  }

  pub fn with_requirements(mut self, requirements: ::models::DestinyDefinitionsPresentationDestinyPresentationNodeRequirementsBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.requirements = Some(requirements);
    self
  }

  pub fn requirements(&self) -> Option<&::models::DestinyDefinitionsPresentationDestinyPresentationNodeRequirementsBlock> {
    self.requirements.as_ref()
  }

  pub fn reset_requirements(&mut self) {
    self.requirements = None;
  }

  pub fn set_expiration_info(&mut self, expiration_info: ::models::DestinyDefinitionsRecordsDestinyRecordExpirationBlock) {
    self.expiration_info = Some(expiration_info);
  }

  pub fn with_expiration_info(mut self, expiration_info: ::models::DestinyDefinitionsRecordsDestinyRecordExpirationBlock) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.expiration_info = Some(expiration_info);
    self
  }

  pub fn expiration_info(&self) -> Option<&::models::DestinyDefinitionsRecordsDestinyRecordExpirationBlock> {
    self.expiration_info.as_ref()
  }

  pub fn reset_expiration_info(&mut self) {
    self.expiration_info = None;
  }

  pub fn set_interval_info(&mut self, interval_info: Value) {
    self.interval_info = Some(interval_info);
  }

  pub fn with_interval_info(mut self, interval_info: Value) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.interval_info = Some(interval_info);
    self
  }

  pub fn interval_info(&self) -> Option<&Value> {
    self.interval_info.as_ref()
  }

  pub fn reset_interval_info(&mut self) {
    self.interval_info = None;
  }

  pub fn set_reward_items(&mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) {
    self.reward_items = Some(reward_items);
  }

  pub fn with_reward_items(mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.reward_items = Some(reward_items);
    self
  }

  pub fn reward_items(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.reward_items.as_ref()
  }

  pub fn reset_reward_items(&mut self) {
    self.reward_items = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsRecordsDestinyRecordDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}


