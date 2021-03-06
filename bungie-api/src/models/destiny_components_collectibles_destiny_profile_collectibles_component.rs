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
pub struct DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
  /// The list of collectibles determined by the game as having been \"recently\" acquired.
  #[serde(rename = "recentCollectibleHashes")]
  recent_collectible_hashes: Option<Vec<i32>>,
  /// The list of collectibles determined by the game as having been \"recently\" acquired.  The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can't edit this value through the API. But in case anyone finds it useful, here it is.
  #[serde(rename = "newnessFlaggedCollectibleHashes")]
  newness_flagged_collectible_hashes: Option<Vec<i32>>,
  #[serde(rename = "collectibles")]
  collectibles: Option<::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>>
}

impl DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
  pub fn new() -> DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
    DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
      recent_collectible_hashes: None,
      newness_flagged_collectible_hashes: None,
      collectibles: None
    }
  }

  pub fn set_recent_collectible_hashes(&mut self, recent_collectible_hashes: Vec<i32>) {
    self.recent_collectible_hashes = Some(recent_collectible_hashes);
  }

  pub fn with_recent_collectible_hashes(mut self, recent_collectible_hashes: Vec<i32>) -> DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
    self.recent_collectible_hashes = Some(recent_collectible_hashes);
    self
  }

  pub fn recent_collectible_hashes(&self) -> Option<&Vec<i32>> {
    self.recent_collectible_hashes.as_ref()
  }

  pub fn reset_recent_collectible_hashes(&mut self) {
    self.recent_collectible_hashes = None;
  }

  pub fn set_newness_flagged_collectible_hashes(&mut self, newness_flagged_collectible_hashes: Vec<i32>) {
    self.newness_flagged_collectible_hashes = Some(newness_flagged_collectible_hashes);
  }

  pub fn with_newness_flagged_collectible_hashes(mut self, newness_flagged_collectible_hashes: Vec<i32>) -> DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
    self.newness_flagged_collectible_hashes = Some(newness_flagged_collectible_hashes);
    self
  }

  pub fn newness_flagged_collectible_hashes(&self) -> Option<&Vec<i32>> {
    self.newness_flagged_collectible_hashes.as_ref()
  }

  pub fn reset_newness_flagged_collectible_hashes(&mut self) {
    self.newness_flagged_collectible_hashes = None;
  }

  pub fn set_collectibles(&mut self, collectibles: ::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>) {
    self.collectibles = Some(collectibles);
  }

  pub fn with_collectibles(mut self, collectibles: ::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>) -> DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent {
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



