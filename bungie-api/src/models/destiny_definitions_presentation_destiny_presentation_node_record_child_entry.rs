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
pub struct DestinyDefinitionsPresentationDestinyPresentationNodeRecordChildEntry {
  #[serde(rename = "recordHash")]
  record_hash: Option<i32>
}

impl DestinyDefinitionsPresentationDestinyPresentationNodeRecordChildEntry {
  pub fn new() -> DestinyDefinitionsPresentationDestinyPresentationNodeRecordChildEntry {
    DestinyDefinitionsPresentationDestinyPresentationNodeRecordChildEntry {
      record_hash: None
    }
  }

  pub fn set_record_hash(&mut self, record_hash: i32) {
    self.record_hash = Some(record_hash);
  }

  pub fn with_record_hash(mut self, record_hash: i32) -> DestinyDefinitionsPresentationDestinyPresentationNodeRecordChildEntry {
    self.record_hash = Some(record_hash);
    self
  }

  pub fn record_hash(&self) -> Option<&i32> {
    self.record_hash.as_ref()
  }

  pub fn reset_record_hash(&mut self) {
    self.record_hash = None;
  }

}



