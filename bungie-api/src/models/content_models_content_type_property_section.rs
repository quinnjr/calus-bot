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
pub struct ContentModelsContentTypePropertySection {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "readableName")]
  readable_name: Option<String>,
  #[serde(rename = "collapsed")]
  collapsed: Option<bool>
}

impl ContentModelsContentTypePropertySection {
  pub fn new() -> ContentModelsContentTypePropertySection {
    ContentModelsContentTypePropertySection {
      name: None,
      readable_name: None,
      collapsed: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ContentModelsContentTypePropertySection {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_readable_name(&mut self, readable_name: String) {
    self.readable_name = Some(readable_name);
  }

  pub fn with_readable_name(mut self, readable_name: String) -> ContentModelsContentTypePropertySection {
    self.readable_name = Some(readable_name);
    self
  }

  pub fn readable_name(&self) -> Option<&String> {
    self.readable_name.as_ref()
  }

  pub fn reset_readable_name(&mut self) {
    self.readable_name = None;
  }

  pub fn set_collapsed(&mut self, collapsed: bool) {
    self.collapsed = Some(collapsed);
  }

  pub fn with_collapsed(mut self, collapsed: bool) -> ContentModelsContentTypePropertySection {
    self.collapsed = Some(collapsed);
    self
  }

  pub fn collapsed(&self) -> Option<&bool> {
    self.collapsed.as_ref()
  }

  pub fn reset_collapsed(&mut self) {
    self.collapsed = None;
  }

}



