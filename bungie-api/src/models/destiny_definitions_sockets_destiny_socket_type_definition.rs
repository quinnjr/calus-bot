/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsSocketsDestinySocketTypeDefinition : All Sockets have a \"Type\": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.  See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsSocketsDestinySocketTypeDefinition {
  /// There are fields for this display data, but they appear to be unpopulated as of now. I am not sure where in the UI these would show if they even were populated, but I will continue to return this data in case it becomes useful.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Value>,
  /// Defines what happens when a plug is inserted into sockets of this type.
  #[serde(rename = "insertAction")]
  insert_action: Option<Value>,
  /// A list of Plug \"Categories\" that are allowed to be plugged into sockets of this type.  These should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.  If the plug's category matches any whitelisted plug, or if the whitelist is empty, it is allowed to be inserted.
  #[serde(rename = "plugWhitelist")]
  plug_whitelist: Option<Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>>,
  #[serde(rename = "socketCategoryHash")]
  socket_category_hash: Option<i32>,
  /// Sometimes a socket isn't visible. These are some of the conditions under which sockets of this type are not visible. Unfortunately, the truth of visibility is much, much more complex. Best to rely on the live data for whether the socket is visible and enabled.
  #[serde(rename = "visibility")]
  visibility: Option<i32>,
  #[serde(rename = "alwaysRandomizeSockets")]
  always_randomize_sockets: Option<bool>,
  #[serde(rename = "isPreviewEnabled")]
  is_preview_enabled: Option<bool>,
  #[serde(rename = "hideDuplicateReusablePlugs")]
  hide_duplicate_reusable_plugs: Option<bool>,
  /// This property indicates if the socket type determines whether Emblem icons and nameplates should be overridden by the inserted plug item's icon and nameplate.
  #[serde(rename = "overridesUiAppearance")]
  overrides_ui_appearance: Option<bool>,
  #[serde(rename = "avoidDuplicatesOnInitialization")]
  avoid_duplicates_on_initialization: Option<bool>,
  #[serde(rename = "currencyScalars")]
  currency_scalars: Option<Vec<::models::DestinyDefinitionsSocketsDestinySocketTypeScalarMaterialRequirementEntry>>,
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

impl DestinyDefinitionsSocketsDestinySocketTypeDefinition {
  /// All Sockets have a \"Type\": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.  See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.
  pub fn new() -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    DestinyDefinitionsSocketsDestinySocketTypeDefinition {
      display_properties: None,
      insert_action: None,
      plug_whitelist: None,
      socket_category_hash: None,
      visibility: None,
      always_randomize_sockets: None,
      is_preview_enabled: None,
      hide_duplicate_reusable_plugs: None,
      overrides_ui_appearance: None,
      avoid_duplicates_on_initialization: None,
      currency_scalars: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: Value) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Value) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Value> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_insert_action(&mut self, insert_action: Value) {
    self.insert_action = Some(insert_action);
  }

  pub fn with_insert_action(mut self, insert_action: Value) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.insert_action = Some(insert_action);
    self
  }

  pub fn insert_action(&self) -> Option<&Value> {
    self.insert_action.as_ref()
  }

  pub fn reset_insert_action(&mut self) {
    self.insert_action = None;
  }

  pub fn set_plug_whitelist(&mut self, plug_whitelist: Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>) {
    self.plug_whitelist = Some(plug_whitelist);
  }

  pub fn with_plug_whitelist(mut self, plug_whitelist: Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.plug_whitelist = Some(plug_whitelist);
    self
  }

  pub fn plug_whitelist(&self) -> Option<&Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>> {
    self.plug_whitelist.as_ref()
  }

  pub fn reset_plug_whitelist(&mut self) {
    self.plug_whitelist = None;
  }

  pub fn set_socket_category_hash(&mut self, socket_category_hash: i32) {
    self.socket_category_hash = Some(socket_category_hash);
  }

  pub fn with_socket_category_hash(mut self, socket_category_hash: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.socket_category_hash = Some(socket_category_hash);
    self
  }

  pub fn socket_category_hash(&self) -> Option<&i32> {
    self.socket_category_hash.as_ref()
  }

  pub fn reset_socket_category_hash(&mut self) {
    self.socket_category_hash = None;
  }

  pub fn set_visibility(&mut self, visibility: i32) {
    self.visibility = Some(visibility);
  }

  pub fn with_visibility(mut self, visibility: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.visibility = Some(visibility);
    self
  }

  pub fn visibility(&self) -> Option<&i32> {
    self.visibility.as_ref()
  }

  pub fn reset_visibility(&mut self) {
    self.visibility = None;
  }

  pub fn set_always_randomize_sockets(&mut self, always_randomize_sockets: bool) {
    self.always_randomize_sockets = Some(always_randomize_sockets);
  }

  pub fn with_always_randomize_sockets(mut self, always_randomize_sockets: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.always_randomize_sockets = Some(always_randomize_sockets);
    self
  }

  pub fn always_randomize_sockets(&self) -> Option<&bool> {
    self.always_randomize_sockets.as_ref()
  }

  pub fn reset_always_randomize_sockets(&mut self) {
    self.always_randomize_sockets = None;
  }

  pub fn set_is_preview_enabled(&mut self, is_preview_enabled: bool) {
    self.is_preview_enabled = Some(is_preview_enabled);
  }

  pub fn with_is_preview_enabled(mut self, is_preview_enabled: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.is_preview_enabled = Some(is_preview_enabled);
    self
  }

  pub fn is_preview_enabled(&self) -> Option<&bool> {
    self.is_preview_enabled.as_ref()
  }

  pub fn reset_is_preview_enabled(&mut self) {
    self.is_preview_enabled = None;
  }

  pub fn set_hide_duplicate_reusable_plugs(&mut self, hide_duplicate_reusable_plugs: bool) {
    self.hide_duplicate_reusable_plugs = Some(hide_duplicate_reusable_plugs);
  }

  pub fn with_hide_duplicate_reusable_plugs(mut self, hide_duplicate_reusable_plugs: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.hide_duplicate_reusable_plugs = Some(hide_duplicate_reusable_plugs);
    self
  }

  pub fn hide_duplicate_reusable_plugs(&self) -> Option<&bool> {
    self.hide_duplicate_reusable_plugs.as_ref()
  }

  pub fn reset_hide_duplicate_reusable_plugs(&mut self) {
    self.hide_duplicate_reusable_plugs = None;
  }

  pub fn set_overrides_ui_appearance(&mut self, overrides_ui_appearance: bool) {
    self.overrides_ui_appearance = Some(overrides_ui_appearance);
  }

  pub fn with_overrides_ui_appearance(mut self, overrides_ui_appearance: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.overrides_ui_appearance = Some(overrides_ui_appearance);
    self
  }

  pub fn overrides_ui_appearance(&self) -> Option<&bool> {
    self.overrides_ui_appearance.as_ref()
  }

  pub fn reset_overrides_ui_appearance(&mut self) {
    self.overrides_ui_appearance = None;
  }

  pub fn set_avoid_duplicates_on_initialization(&mut self, avoid_duplicates_on_initialization: bool) {
    self.avoid_duplicates_on_initialization = Some(avoid_duplicates_on_initialization);
  }

  pub fn with_avoid_duplicates_on_initialization(mut self, avoid_duplicates_on_initialization: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.avoid_duplicates_on_initialization = Some(avoid_duplicates_on_initialization);
    self
  }

  pub fn avoid_duplicates_on_initialization(&self) -> Option<&bool> {
    self.avoid_duplicates_on_initialization.as_ref()
  }

  pub fn reset_avoid_duplicates_on_initialization(&mut self) {
    self.avoid_duplicates_on_initialization = None;
  }

  pub fn set_currency_scalars(&mut self, currency_scalars: Vec<::models::DestinyDefinitionsSocketsDestinySocketTypeScalarMaterialRequirementEntry>) {
    self.currency_scalars = Some(currency_scalars);
  }

  pub fn with_currency_scalars(mut self, currency_scalars: Vec<::models::DestinyDefinitionsSocketsDestinySocketTypeScalarMaterialRequirementEntry>) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.currency_scalars = Some(currency_scalars);
    self
  }

  pub fn currency_scalars(&self) -> Option<&Vec<::models::DestinyDefinitionsSocketsDestinySocketTypeScalarMaterialRequirementEntry>> {
    self.currency_scalars.as_ref()
  }

  pub fn reset_currency_scalars(&mut self) {
    self.currency_scalars = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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



