/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDestinyUnlockStatus : Indicates the status of an \"Unlock Flag\" on a Character or Profile.  These are individual bits of state that can be either set or not set, and sometimes provide interesting human-readable information in their related DestinyUnlockDefinition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDestinyUnlockStatus {
  /// The hash identifier for the Unlock Flag. Use to lookup DestinyUnlockDefinition for static data. Not all unlocks have human readable data - in fact, most don't. But when they do, it can be very useful to show. Even if they don't have human readable data, you might be able to infer the meaning of an unlock flag with a bit of experimentation...
  #[serde(rename = "unlockHash")]
  unlock_hash: Option<i32>,
  /// Whether the unlock flag is set.
  #[serde(rename = "isSet")]
  is_set: Option<bool>
}

impl DestinyDestinyUnlockStatus {
  /// Indicates the status of an \"Unlock Flag\" on a Character or Profile.  These are individual bits of state that can be either set or not set, and sometimes provide interesting human-readable information in their related DestinyUnlockDefinition.
  pub fn new() -> DestinyDestinyUnlockStatus {
    DestinyDestinyUnlockStatus {
      unlock_hash: None,
      is_set: None
    }
  }

  pub fn set_unlock_hash(&mut self, unlock_hash: i32) {
    self.unlock_hash = Some(unlock_hash);
  }

  pub fn with_unlock_hash(mut self, unlock_hash: i32) -> DestinyDestinyUnlockStatus {
    self.unlock_hash = Some(unlock_hash);
    self
  }

  pub fn unlock_hash(&self) -> Option<&i32> {
    self.unlock_hash.as_ref()
  }

  pub fn reset_unlock_hash(&mut self) {
    self.unlock_hash = None;
  }

  pub fn set_is_set(&mut self, is_set: bool) {
    self.is_set = Some(is_set);
  }

  pub fn with_is_set(mut self, is_set: bool) -> DestinyDestinyUnlockStatus {
    self.is_set = Some(is_set);
    self
  }

  pub fn is_set(&self) -> Option<&bool> {
    self.is_set.as_ref()
  }

  pub fn reset_is_set(&mut self) {
    self.is_set = None;
  }

}



