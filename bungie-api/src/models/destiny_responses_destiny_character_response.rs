/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyResponsesDestinyCharacterResponse : The response contract for GetDestinyCharacter, with components that can be returned for character and item-level data.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyCharacterResponse {
  /// The character-level non-equipped inventory items.  COMPONENT TYPE: CharacterInventories
  #[serde(rename = "inventory")]
  inventory: Option<Value>,
  /// Base information about the character in question.  COMPONENT TYPE: Characters
  #[serde(rename = "character")]
  character: Option<Value>,
  /// Character progression data, including Milestones.  COMPONENT TYPE: CharacterProgressions
  #[serde(rename = "progressions")]
  progressions: Option<Value>,
  /// Character rendering data - a minimal set of information about equipment and dyes used for rendering.  COMPONENT TYPE: CharacterRenderData
  #[serde(rename = "renderData")]
  render_data: Option<Value>,
  /// Activity data - info about current activities available to the player.  COMPONENT TYPE: CharacterActivities
  #[serde(rename = "activities")]
  activities: Option<Value>,
  /// Equipped items on the character.  COMPONENT TYPE: CharacterEquipment
  #[serde(rename = "equipment")]
  equipment: Option<Value>,
  /// Items available from Kiosks that are available to this specific character.   COMPONENT TYPE: Kiosks
  #[serde(rename = "kiosks")]
  kiosks: Option<Value>,
  /// When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are scoped to this character.  This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.  COMPONENT TYPE: ItemSockets
  #[serde(rename = "plugSets")]
  plug_sets: Option<Value>,
  /// COMPONENT TYPE: PresentationNodes
  #[serde(rename = "presentationNodes")]
  presentation_nodes: Option<Value>,
  /// COMPONENT TYPE: Records
  #[serde(rename = "records")]
  records: Option<Value>,
  /// COMPONENT TYPE: Collectibles
  #[serde(rename = "collectibles")]
  collectibles: Option<Value>,
  /// The set of components belonging to the player's instanced items.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
  #[serde(rename = "itemComponents")]
  item_components: Option<Value>,
  /// The set of components belonging to the player's UNinstanced items. Because apparently now those too can have information relevant to the character's state.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
  #[serde(rename = "uninstancedItemComponents")]
  uninstanced_item_components: Option<Value>,
  /// A \"lookup\" convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.  COMPONENT TYPE: CurrencyLookups
  #[serde(rename = "currencyLookups")]
  currency_lookups: Option<Value>
}

impl DestinyResponsesDestinyCharacterResponse {
  /// The response contract for GetDestinyCharacter, with components that can be returned for character and item-level data.
  pub fn new() -> DestinyResponsesDestinyCharacterResponse {
    DestinyResponsesDestinyCharacterResponse {
      inventory: None,
      character: None,
      progressions: None,
      render_data: None,
      activities: None,
      equipment: None,
      kiosks: None,
      plug_sets: None,
      presentation_nodes: None,
      records: None,
      collectibles: None,
      item_components: None,
      uninstanced_item_components: None,
      currency_lookups: None
    }
  }

  pub fn set_inventory(&mut self, inventory: Value) {
    self.inventory = Some(inventory);
  }

  pub fn with_inventory(mut self, inventory: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.inventory = Some(inventory);
    self
  }

  pub fn inventory(&self) -> Option<&Value> {
    self.inventory.as_ref()
  }

  pub fn reset_inventory(&mut self) {
    self.inventory = None;
  }

  pub fn set_character(&mut self, character: Value) {
    self.character = Some(character);
  }

  pub fn with_character(mut self, character: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.character = Some(character);
    self
  }

  pub fn character(&self) -> Option<&Value> {
    self.character.as_ref()
  }

  pub fn reset_character(&mut self) {
    self.character = None;
  }

  pub fn set_progressions(&mut self, progressions: Value) {
    self.progressions = Some(progressions);
  }

  pub fn with_progressions(mut self, progressions: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.progressions = Some(progressions);
    self
  }

  pub fn progressions(&self) -> Option<&Value> {
    self.progressions.as_ref()
  }

  pub fn reset_progressions(&mut self) {
    self.progressions = None;
  }

  pub fn set_render_data(&mut self, render_data: Value) {
    self.render_data = Some(render_data);
  }

  pub fn with_render_data(mut self, render_data: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.render_data = Some(render_data);
    self
  }

  pub fn render_data(&self) -> Option<&Value> {
    self.render_data.as_ref()
  }

  pub fn reset_render_data(&mut self) {
    self.render_data = None;
  }

  pub fn set_activities(&mut self, activities: Value) {
    self.activities = Some(activities);
  }

  pub fn with_activities(mut self, activities: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.activities = Some(activities);
    self
  }

  pub fn activities(&self) -> Option<&Value> {
    self.activities.as_ref()
  }

  pub fn reset_activities(&mut self) {
    self.activities = None;
  }

  pub fn set_equipment(&mut self, equipment: Value) {
    self.equipment = Some(equipment);
  }

  pub fn with_equipment(mut self, equipment: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.equipment = Some(equipment);
    self
  }

  pub fn equipment(&self) -> Option<&Value> {
    self.equipment.as_ref()
  }

  pub fn reset_equipment(&mut self) {
    self.equipment = None;
  }

  pub fn set_kiosks(&mut self, kiosks: Value) {
    self.kiosks = Some(kiosks);
  }

  pub fn with_kiosks(mut self, kiosks: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.kiosks = Some(kiosks);
    self
  }

  pub fn kiosks(&self) -> Option<&Value> {
    self.kiosks.as_ref()
  }

  pub fn reset_kiosks(&mut self) {
    self.kiosks = None;
  }

  pub fn set_plug_sets(&mut self, plug_sets: Value) {
    self.plug_sets = Some(plug_sets);
  }

  pub fn with_plug_sets(mut self, plug_sets: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.plug_sets = Some(plug_sets);
    self
  }

  pub fn plug_sets(&self) -> Option<&Value> {
    self.plug_sets.as_ref()
  }

  pub fn reset_plug_sets(&mut self) {
    self.plug_sets = None;
  }

  pub fn set_presentation_nodes(&mut self, presentation_nodes: Value) {
    self.presentation_nodes = Some(presentation_nodes);
  }

  pub fn with_presentation_nodes(mut self, presentation_nodes: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.presentation_nodes = Some(presentation_nodes);
    self
  }

  pub fn presentation_nodes(&self) -> Option<&Value> {
    self.presentation_nodes.as_ref()
  }

  pub fn reset_presentation_nodes(&mut self) {
    self.presentation_nodes = None;
  }

  pub fn set_records(&mut self, records: Value) {
    self.records = Some(records);
  }

  pub fn with_records(mut self, records: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.records = Some(records);
    self
  }

  pub fn records(&self) -> Option<&Value> {
    self.records.as_ref()
  }

  pub fn reset_records(&mut self) {
    self.records = None;
  }

  pub fn set_collectibles(&mut self, collectibles: Value) {
    self.collectibles = Some(collectibles);
  }

  pub fn with_collectibles(mut self, collectibles: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.collectibles = Some(collectibles);
    self
  }

  pub fn collectibles(&self) -> Option<&Value> {
    self.collectibles.as_ref()
  }

  pub fn reset_collectibles(&mut self) {
    self.collectibles = None;
  }

  pub fn set_item_components(&mut self, item_components: Value) {
    self.item_components = Some(item_components);
  }

  pub fn with_item_components(mut self, item_components: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.item_components = Some(item_components);
    self
  }

  pub fn item_components(&self) -> Option<&Value> {
    self.item_components.as_ref()
  }

  pub fn reset_item_components(&mut self) {
    self.item_components = None;
  }

  pub fn set_uninstanced_item_components(&mut self, uninstanced_item_components: Value) {
    self.uninstanced_item_components = Some(uninstanced_item_components);
  }

  pub fn with_uninstanced_item_components(mut self, uninstanced_item_components: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.uninstanced_item_components = Some(uninstanced_item_components);
    self
  }

  pub fn uninstanced_item_components(&self) -> Option<&Value> {
    self.uninstanced_item_components.as_ref()
  }

  pub fn reset_uninstanced_item_components(&mut self) {
    self.uninstanced_item_components = None;
  }

  pub fn set_currency_lookups(&mut self, currency_lookups: Value) {
    self.currency_lookups = Some(currency_lookups);
  }

  pub fn with_currency_lookups(mut self, currency_lookups: Value) -> DestinyResponsesDestinyCharacterResponse {
    self.currency_lookups = Some(currency_lookups);
    self
  }

  pub fn currency_lookups(&self) -> Option<&Value> {
    self.currency_lookups.as_ref()
  }

  pub fn reset_currency_lookups(&mut self) {
    self.currency_lookups = None;
  }

}



