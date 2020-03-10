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
pub struct EntitiesEntityActionResult {
  #[serde(rename = "entityId")]
  entity_id: Option<i64>,
  #[serde(rename = "result")]
  result: Option<i32>
}

impl EntitiesEntityActionResult {
  pub fn new() -> EntitiesEntityActionResult {
    EntitiesEntityActionResult {
      entity_id: None,
      result: None
    }
  }

  pub fn set_entity_id(&mut self, entity_id: i64) {
    self.entity_id = Some(entity_id);
  }

  pub fn with_entity_id(mut self, entity_id: i64) -> EntitiesEntityActionResult {
    self.entity_id = Some(entity_id);
    self
  }

  pub fn entity_id(&self) -> Option<&i64> {
    self.entity_id.as_ref()
  }

  pub fn reset_entity_id(&mut self) {
    self.entity_id = None;
  }

  pub fn set_result(&mut self, result: i32) {
    self.result = Some(result);
  }

  pub fn with_result(mut self, result: i32) -> EntitiesEntityActionResult {
    self.result = Some(result);
    self
  }

  pub fn result(&self) -> Option<&i32> {
    self.result.as_ref()
  }

  pub fn reset_result(&mut self) {
    self.result = None;
  }

}



