/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesItemsDestinyItemRenderComponent : Many items can be rendered in 3D. When you request this block, you will obtain the custom data needed to render this specific instance of the item.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesItemsDestinyItemRenderComponent {
  /// If you should use custom dyes on this item, it will be indicated here.
  #[serde(rename = "useCustomDyes")]
  use_custom_dyes: Option<bool>,
  /// A dictionary for rendering gear components, with:  key = Art Arrangement Region Index  value = The chosen Arrangement Index for the Region, based on the value of a stat on the item used for making the choice.
  #[serde(rename = "artRegions")]
  art_regions: Option<::std::collections::HashMap<String, i32>>
}

impl DestinyEntitiesItemsDestinyItemRenderComponent {
  /// Many items can be rendered in 3D. When you request this block, you will obtain the custom data needed to render this specific instance of the item.
  pub fn new() -> DestinyEntitiesItemsDestinyItemRenderComponent {
    DestinyEntitiesItemsDestinyItemRenderComponent {
      use_custom_dyes: None,
      art_regions: None
    }
  }

  pub fn set_use_custom_dyes(&mut self, use_custom_dyes: bool) {
    self.use_custom_dyes = Some(use_custom_dyes);
  }

  pub fn with_use_custom_dyes(mut self, use_custom_dyes: bool) -> DestinyEntitiesItemsDestinyItemRenderComponent {
    self.use_custom_dyes = Some(use_custom_dyes);
    self
  }

  pub fn use_custom_dyes(&self) -> Option<&bool> {
    self.use_custom_dyes.as_ref()
  }

  pub fn reset_use_custom_dyes(&mut self) {
    self.use_custom_dyes = None;
  }

  pub fn set_art_regions(&mut self, art_regions: ::std::collections::HashMap<String, i32>) {
    self.art_regions = Some(art_regions);
  }

  pub fn with_art_regions(mut self, art_regions: ::std::collections::HashMap<String, i32>) -> DestinyEntitiesItemsDestinyItemRenderComponent {
    self.art_regions = Some(art_regions);
    self
  }

  pub fn art_regions(&self) -> Option<&::std::collections::HashMap<String, i32>> {
    self.art_regions.as_ref()
  }

  pub fn reset_art_regions(&mut self) {
    self.art_regions = None;
  }

}



