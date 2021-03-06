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
pub struct DestinyComponentsInventoryDestinyPlatformSilverComponent {
  /// If a Profile is played on multiple platforms, this is the silver they have for each platform, keyed by Membership Type.
  #[serde(rename = "platformSilver")]
  platform_silver: Option<::std::collections::HashMap<String, ::models::DestinyEntitiesItemsDestinyItemComponent>>
}

impl DestinyComponentsInventoryDestinyPlatformSilverComponent {
  pub fn new() -> DestinyComponentsInventoryDestinyPlatformSilverComponent {
    DestinyComponentsInventoryDestinyPlatformSilverComponent {
      platform_silver: None
    }
  }

  pub fn set_platform_silver(&mut self, platform_silver: ::std::collections::HashMap<String, ::models::DestinyEntitiesItemsDestinyItemComponent>) {
    self.platform_silver = Some(platform_silver);
  }

  pub fn with_platform_silver(mut self, platform_silver: ::std::collections::HashMap<String, ::models::DestinyEntitiesItemsDestinyItemComponent>) -> DestinyComponentsInventoryDestinyPlatformSilverComponent {
    self.platform_silver = Some(platform_silver);
    self
  }

  pub fn platform_silver(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyEntitiesItemsDestinyItemComponent>> {
    self.platform_silver.as_ref()
  }

  pub fn reset_platform_silver(&mut self) {
    self.platform_silver = None;
  }

}



