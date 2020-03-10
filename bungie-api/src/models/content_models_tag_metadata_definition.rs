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
pub struct ContentModelsTagMetadataDefinition {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "order")]
  order: Option<i32>,
  #[serde(rename = "items")]
  items: Option<Vec<::models::ContentModelsTagMetadataItem>>,
  #[serde(rename = "datatype")]
  datatype: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "isRequired")]
  is_required: Option<bool>
}

impl ContentModelsTagMetadataDefinition {
  pub fn new() -> ContentModelsTagMetadataDefinition {
    ContentModelsTagMetadataDefinition {
      description: None,
      order: None,
      items: None,
      datatype: None,
      name: None,
      is_required: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> ContentModelsTagMetadataDefinition {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_order(&mut self, order: i32) {
    self.order = Some(order);
  }

  pub fn with_order(mut self, order: i32) -> ContentModelsTagMetadataDefinition {
    self.order = Some(order);
    self
  }

  pub fn order(&self) -> Option<&i32> {
    self.order.as_ref()
  }

  pub fn reset_order(&mut self) {
    self.order = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::ContentModelsTagMetadataItem>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::ContentModelsTagMetadataItem>) -> ContentModelsTagMetadataDefinition {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::ContentModelsTagMetadataItem>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_datatype(&mut self, datatype: String) {
    self.datatype = Some(datatype);
  }

  pub fn with_datatype(mut self, datatype: String) -> ContentModelsTagMetadataDefinition {
    self.datatype = Some(datatype);
    self
  }

  pub fn datatype(&self) -> Option<&String> {
    self.datatype.as_ref()
  }

  pub fn reset_datatype(&mut self) {
    self.datatype = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ContentModelsTagMetadataDefinition {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_is_required(&mut self, is_required: bool) {
    self.is_required = Some(is_required);
  }

  pub fn with_is_required(mut self, is_required: bool) -> ContentModelsTagMetadataDefinition {
    self.is_required = Some(is_required);
    self
  }

  pub fn is_required(&self) -> Option<&bool> {
    self.is_required.as_ref()
  }

  pub fn reset_is_required(&mut self) {
    self.is_required = None;
  }

}


