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
pub struct DestinyComponentsCollectiblesDestinyCollectiblesComponent {
  #[serde(rename = "collectibles")]
  collectibles: Option<::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>>
}

impl DestinyComponentsCollectiblesDestinyCollectiblesComponent {
  pub fn new() -> DestinyComponentsCollectiblesDestinyCollectiblesComponent {
    DestinyComponentsCollectiblesDestinyCollectiblesComponent {
      collectibles: None
    }
  }

  pub fn set_collectibles(&mut self, collectibles: ::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>) {
    self.collectibles = Some(collectibles);
  }

  pub fn with_collectibles(mut self, collectibles: ::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>) -> DestinyComponentsCollectiblesDestinyCollectiblesComponent {
    self.collectibles = Some(collectibles);
    self
  }

  pub fn collectibles(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>> {
    self.collectibles.as_ref()
  }

  pub fn reset_collectibles(&mut self) {
    self.collectibles = None;
  }

}



