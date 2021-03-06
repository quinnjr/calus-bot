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
pub struct CommonModelsCoreSetting {
  #[serde(rename = "identifier")]
  identifier: Option<String>,
  #[serde(rename = "isDefault")]
  is_default: Option<bool>,
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  #[serde(rename = "summary")]
  summary: Option<String>,
  #[serde(rename = "imagePath")]
  image_path: Option<String>,
  #[serde(rename = "childSettings")]
  child_settings: Option<Vec<::models::CommonModelsCoreSetting>>
}

impl CommonModelsCoreSetting {
  pub fn new() -> CommonModelsCoreSetting {
    CommonModelsCoreSetting {
      identifier: None,
      is_default: None,
      display_name: None,
      summary: None,
      image_path: None,
      child_settings: None
    }
  }

  pub fn set_identifier(&mut self, identifier: String) {
    self.identifier = Some(identifier);
  }

  pub fn with_identifier(mut self, identifier: String) -> CommonModelsCoreSetting {
    self.identifier = Some(identifier);
    self
  }

  pub fn identifier(&self) -> Option<&String> {
    self.identifier.as_ref()
  }

  pub fn reset_identifier(&mut self) {
    self.identifier = None;
  }

  pub fn set_is_default(&mut self, is_default: bool) {
    self.is_default = Some(is_default);
  }

  pub fn with_is_default(mut self, is_default: bool) -> CommonModelsCoreSetting {
    self.is_default = Some(is_default);
    self
  }

  pub fn is_default(&self) -> Option<&bool> {
    self.is_default.as_ref()
  }

  pub fn reset_is_default(&mut self) {
    self.is_default = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> CommonModelsCoreSetting {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_summary(&mut self, summary: String) {
    self.summary = Some(summary);
  }

  pub fn with_summary(mut self, summary: String) -> CommonModelsCoreSetting {
    self.summary = Some(summary);
    self
  }

  pub fn summary(&self) -> Option<&String> {
    self.summary.as_ref()
  }

  pub fn reset_summary(&mut self) {
    self.summary = None;
  }

  pub fn set_image_path(&mut self, image_path: String) {
    self.image_path = Some(image_path);
  }

  pub fn with_image_path(mut self, image_path: String) -> CommonModelsCoreSetting {
    self.image_path = Some(image_path);
    self
  }

  pub fn image_path(&self) -> Option<&String> {
    self.image_path.as_ref()
  }

  pub fn reset_image_path(&mut self) {
    self.image_path = None;
  }

  pub fn set_child_settings(&mut self, child_settings: Vec<::models::CommonModelsCoreSetting>) {
    self.child_settings = Some(child_settings);
  }

  pub fn with_child_settings(mut self, child_settings: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSetting {
    self.child_settings = Some(child_settings);
    self
  }

  pub fn child_settings(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.child_settings.as_ref()
  }

  pub fn reset_child_settings(&mut self) {
    self.child_settings = None;
  }

}



