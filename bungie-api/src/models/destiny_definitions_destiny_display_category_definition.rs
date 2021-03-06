/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyDisplayCategoryDefinition : Display Categories are different from \"categories\" in that these are specifically for visual grouping and display of categories in Vendor UI. The \"categories\" structure is for validation of the contained items, and can be categorized entirely separately from \"Display Categories\", there need be and often will be no meaningful relationship between the two.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyDisplayCategoryDefinition {
  #[serde(rename = "index")]
  index: Option<i32>,
  /// A string identifier for the display category.
  #[serde(rename = "identifier")]
  identifier: Option<String>,
  #[serde(rename = "displayCategoryHash")]
  display_category_hash: Option<i32>,
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// If true, this category should be displayed in the \"Banner\" section of the vendor's UI.
  #[serde(rename = "displayInBanner")]
  display_in_banner: Option<bool>,
  /// If it exists, this is the hash identifier of a DestinyProgressionDefinition that represents the progression to show on this display category.  Specific categories can now have thier own distinct progression, apparently. So that's cool.
  #[serde(rename = "progressionHash")]
  progression_hash: Option<i32>,
  /// If this category sorts items in a nonstandard way, this will be the way we sort.
  #[serde(rename = "sortOrder")]
  sort_order: Option<i32>,
  /// An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category.
  #[serde(rename = "displayStyleHash")]
  display_style_hash: Option<i32>,
  /// An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category.
  #[serde(rename = "displayStyleIdentifier")]
  display_style_identifier: Option<String>
}

impl DestinyDefinitionsDestinyDisplayCategoryDefinition {
  /// Display Categories are different from \"categories\" in that these are specifically for visual grouping and display of categories in Vendor UI. The \"categories\" structure is for validation of the contained items, and can be categorized entirely separately from \"Display Categories\", there need be and often will be no meaningful relationship between the two.
  pub fn new() -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    DestinyDefinitionsDestinyDisplayCategoryDefinition {
      index: None,
      identifier: None,
      display_category_hash: None,
      display_properties: None,
      display_in_banner: None,
      progression_hash: None,
      sort_order: None,
      display_style_hash: None,
      display_style_identifier: None
    }
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_identifier(&mut self, identifier: String) {
    self.identifier = Some(identifier);
  }

  pub fn with_identifier(mut self, identifier: String) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.identifier = Some(identifier);
    self
  }

  pub fn identifier(&self) -> Option<&String> {
    self.identifier.as_ref()
  }

  pub fn reset_identifier(&mut self) {
    self.identifier = None;
  }

  pub fn set_display_category_hash(&mut self, display_category_hash: i32) {
    self.display_category_hash = Some(display_category_hash);
  }

  pub fn with_display_category_hash(mut self, display_category_hash: i32) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.display_category_hash = Some(display_category_hash);
    self
  }

  pub fn display_category_hash(&self) -> Option<&i32> {
    self.display_category_hash.as_ref()
  }

  pub fn reset_display_category_hash(&mut self) {
    self.display_category_hash = None;
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_display_in_banner(&mut self, display_in_banner: bool) {
    self.display_in_banner = Some(display_in_banner);
  }

  pub fn with_display_in_banner(mut self, display_in_banner: bool) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.display_in_banner = Some(display_in_banner);
    self
  }

  pub fn display_in_banner(&self) -> Option<&bool> {
    self.display_in_banner.as_ref()
  }

  pub fn reset_display_in_banner(&mut self) {
    self.display_in_banner = None;
  }

  pub fn set_progression_hash(&mut self, progression_hash: i32) {
    self.progression_hash = Some(progression_hash);
  }

  pub fn with_progression_hash(mut self, progression_hash: i32) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.progression_hash = Some(progression_hash);
    self
  }

  pub fn progression_hash(&self) -> Option<&i32> {
    self.progression_hash.as_ref()
  }

  pub fn reset_progression_hash(&mut self) {
    self.progression_hash = None;
  }

  pub fn set_sort_order(&mut self, sort_order: i32) {
    self.sort_order = Some(sort_order);
  }

  pub fn with_sort_order(mut self, sort_order: i32) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.sort_order = Some(sort_order);
    self
  }

  pub fn sort_order(&self) -> Option<&i32> {
    self.sort_order.as_ref()
  }

  pub fn reset_sort_order(&mut self) {
    self.sort_order = None;
  }

  pub fn set_display_style_hash(&mut self, display_style_hash: i32) {
    self.display_style_hash = Some(display_style_hash);
  }

  pub fn with_display_style_hash(mut self, display_style_hash: i32) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.display_style_hash = Some(display_style_hash);
    self
  }

  pub fn display_style_hash(&self) -> Option<&i32> {
    self.display_style_hash.as_ref()
  }

  pub fn reset_display_style_hash(&mut self) {
    self.display_style_hash = None;
  }

  pub fn set_display_style_identifier(&mut self, display_style_identifier: String) {
    self.display_style_identifier = Some(display_style_identifier);
  }

  pub fn with_display_style_identifier(mut self, display_style_identifier: String) -> DestinyDefinitionsDestinyDisplayCategoryDefinition {
    self.display_style_identifier = Some(display_style_identifier);
    self
  }

  pub fn display_style_identifier(&self) -> Option<&String> {
    self.display_style_identifier.as_ref()
  }

  pub fn reset_display_style_identifier(&mut self) {
    self.display_style_identifier = None;
  }

}



