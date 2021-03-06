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
pub struct ConfigGroupTheme {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "folder")]
  folder: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>
}

impl ConfigGroupTheme {
  pub fn new() -> ConfigGroupTheme {
    ConfigGroupTheme {
      name: None,
      folder: None,
      description: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ConfigGroupTheme {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_folder(&mut self, folder: String) {
    self.folder = Some(folder);
  }

  pub fn with_folder(mut self, folder: String) -> ConfigGroupTheme {
    self.folder = Some(folder);
    self
  }

  pub fn folder(&self) -> Option<&String> {
    self.folder.as_ref()
  }

  pub fn reset_folder(&mut self) {
    self.folder = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> ConfigGroupTheme {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

}



