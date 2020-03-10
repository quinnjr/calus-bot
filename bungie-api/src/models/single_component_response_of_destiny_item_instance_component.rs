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
pub struct SingleComponentResponseOfDestinyItemInstanceComponent {
  #[serde(rename = "data")]
  data: Option<::models::DestinyEntitiesItemsDestinyItemInstanceComponent>,
  #[serde(rename = "privacy")]
  privacy: Option<i32>,
  /// If true, this component is disabled.
  #[serde(rename = "disabled")]
  disabled: Option<bool>
}

impl SingleComponentResponseOfDestinyItemInstanceComponent {
  pub fn new() -> SingleComponentResponseOfDestinyItemInstanceComponent {
    SingleComponentResponseOfDestinyItemInstanceComponent {
      data: None,
      privacy: None,
      disabled: None
    }
  }

  pub fn set_data(&mut self, data: ::models::DestinyEntitiesItemsDestinyItemInstanceComponent) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::DestinyEntitiesItemsDestinyItemInstanceComponent) -> SingleComponentResponseOfDestinyItemInstanceComponent {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::DestinyEntitiesItemsDestinyItemInstanceComponent> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_privacy(&mut self, privacy: i32) {
    self.privacy = Some(privacy);
  }

  pub fn with_privacy(mut self, privacy: i32) -> SingleComponentResponseOfDestinyItemInstanceComponent {
    self.privacy = Some(privacy);
    self
  }

  pub fn privacy(&self) -> Option<&i32> {
    self.privacy.as_ref()
  }

  pub fn reset_privacy(&mut self) {
    self.privacy = None;
  }

  pub fn set_disabled(&mut self, disabled: bool) {
    self.disabled = Some(disabled);
  }

  pub fn with_disabled(mut self, disabled: bool) -> SingleComponentResponseOfDestinyItemInstanceComponent {
    self.disabled = Some(disabled);
    self
  }

  pub fn disabled(&self) -> Option<&bool> {
    self.disabled.as_ref()
  }

  pub fn reset_disabled(&mut self) {
    self.disabled = None;
  }

}



