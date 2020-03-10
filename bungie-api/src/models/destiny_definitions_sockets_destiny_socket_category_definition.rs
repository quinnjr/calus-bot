/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsSocketsDestinySocketCategoryDefinition : Sockets on an item are organized into Categories visually.  You can find references to the socket category defined on an item's DestinyInventoryItemDefinition.sockets.socketCategories property.  This has the display information for rendering the categories' header, and a hint for how the UI should handle showing this category.  The shitty thing about this, however, is that the socket categories' UI style can be overridden by the item's UI style. For instance, the Socket Category used by Emote Sockets says it's \"consumable,\" but that's a lie: they're all reusable, and overridden by the detail UI pages in ways that we can't easily account for in the API.  As a result, I will try to compile these rules into the individual sockets on items, and provide the best hint possible there through the plugSources property. In the future, I may attempt to use this information in conjunction with the item to provide a more usable UI hint on the socket layer, but for now improving the consistency of plugSources is the best I have time to provide. (See https://github.com/Bungie-net/api/issues/522 for more info)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// A string hinting to the game's UI system about how the sockets in this category should be displayed.  BNet doesn't use it: it's up to you to find valid values and make your own special UI if you want to honor this category style.
  #[serde(rename = "uiCategoryStyle")]
  ui_category_style: Option<i32>,
  /// Same as uiCategoryStyle, but in a more usable enumeration form.
  #[serde(rename = "categoryStyle")]
  category_style: Option<i32>,
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

impl DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
  /// Sockets on an item are organized into Categories visually.  You can find references to the socket category defined on an item's DestinyInventoryItemDefinition.sockets.socketCategories property.  This has the display information for rendering the categories' header, and a hint for how the UI should handle showing this category.  The shitty thing about this, however, is that the socket categories' UI style can be overridden by the item's UI style. For instance, the Socket Category used by Emote Sockets says it's \"consumable,\" but that's a lie: they're all reusable, and overridden by the detail UI pages in ways that we can't easily account for in the API.  As a result, I will try to compile these rules into the individual sockets on items, and provide the best hint possible there through the plugSources property. In the future, I may attempt to use this information in conjunction with the item to provide a more usable UI hint on the socket layer, but for now improving the consistency of plugSources is the best I have time to provide. (See https://github.com/Bungie-net/api/issues/522 for more info)
  pub fn new() -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
    DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
      display_properties: None,
      ui_category_style: None,
      category_style: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_ui_category_style(&mut self, ui_category_style: i32) {
    self.ui_category_style = Some(ui_category_style);
  }

  pub fn with_ui_category_style(mut self, ui_category_style: i32) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
    self.ui_category_style = Some(ui_category_style);
    self
  }

  pub fn ui_category_style(&self) -> Option<&i32> {
    self.ui_category_style.as_ref()
  }

  pub fn reset_ui_category_style(&mut self) {
    self.ui_category_style = None;
  }

  pub fn set_category_style(&mut self, category_style: i32) {
    self.category_style = Some(category_style);
  }

  pub fn with_category_style(mut self, category_style: i32) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
    self.category_style = Some(category_style);
    self
  }

  pub fn category_style(&self) -> Option<&i32> {
    self.category_style.as_ref()
  }

  pub fn reset_category_style(&mut self) {
    self.category_style = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
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

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
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

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsSocketsDestinySocketCategoryDefinition {
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



