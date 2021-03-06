/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UserEmailViewDefinition : Represents a data-driven view for Email settings. Web/Mobile UI can use this data to show new EMail settings consistently without further manual work.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEmailViewDefinition {
  /// The identifier for this view.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The ordered list of settings to show in this view.
  #[serde(rename = "viewSettings")]
  view_settings: Option<Vec<::models::UserEmailViewDefinitionSetting>>
}

impl UserEmailViewDefinition {
  /// Represents a data-driven view for Email settings. Web/Mobile UI can use this data to show new EMail settings consistently without further manual work.
  pub fn new() -> UserEmailViewDefinition {
    UserEmailViewDefinition {
      name: None,
      view_settings: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> UserEmailViewDefinition {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_view_settings(&mut self, view_settings: Vec<::models::UserEmailViewDefinitionSetting>) {
    self.view_settings = Some(view_settings);
  }

  pub fn with_view_settings(mut self, view_settings: Vec<::models::UserEmailViewDefinitionSetting>) -> UserEmailViewDefinition {
    self.view_settings = Some(view_settings);
    self
  }

  pub fn view_settings(&self) -> Option<&Vec<::models::UserEmailViewDefinitionSetting>> {
    self.view_settings.as_ref()
  }

  pub fn reset_view_settings(&mut self) {
    self.view_settings = None;
  }

}



