/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsRecordsDestinyRecordExpirationBlock : If this record has an expiration after which it cannot be earned, this is some information about that expiration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
  #[serde(rename = "hasExpiration")]
  has_expiration: Option<bool>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "icon")]
  icon: Option<String>
}

impl DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
  /// If this record has an expiration after which it cannot be earned, this is some information about that expiration.
  pub fn new() -> DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
    DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
      has_expiration: None,
      description: None,
      icon: None
    }
  }

  pub fn set_has_expiration(&mut self, has_expiration: bool) {
    self.has_expiration = Some(has_expiration);
  }

  pub fn with_has_expiration(mut self, has_expiration: bool) -> DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
    self.has_expiration = Some(has_expiration);
    self
  }

  pub fn has_expiration(&self) -> Option<&bool> {
    self.has_expiration.as_ref()
  }

  pub fn reset_has_expiration(&mut self) {
    self.has_expiration = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_icon(&mut self, icon: String) {
    self.icon = Some(icon);
  }

  pub fn with_icon(mut self, icon: String) -> DestinyDefinitionsRecordsDestinyRecordExpirationBlock {
    self.icon = Some(icon);
    self
  }

  pub fn icon(&self) -> Option<&String> {
    self.icon.as_ref()
  }

  pub fn reset_icon(&mut self) {
    self.icon = None;
  }

}



