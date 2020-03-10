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
pub struct DestinyDefinitionsDirectorDestinyLinkedGraphEntryDefinition {
  #[serde(rename = "activityGraphHash")]
  activity_graph_hash: Option<i32>
}

impl DestinyDefinitionsDirectorDestinyLinkedGraphEntryDefinition {
  pub fn new() -> DestinyDefinitionsDirectorDestinyLinkedGraphEntryDefinition {
    DestinyDefinitionsDirectorDestinyLinkedGraphEntryDefinition {
      activity_graph_hash: None
    }
  }

  pub fn set_activity_graph_hash(&mut self, activity_graph_hash: i32) {
    self.activity_graph_hash = Some(activity_graph_hash);
  }

  pub fn with_activity_graph_hash(mut self, activity_graph_hash: i32) -> DestinyDefinitionsDirectorDestinyLinkedGraphEntryDefinition {
    self.activity_graph_hash = Some(activity_graph_hash);
    self
  }

  pub fn activity_graph_hash(&self) -> Option<&i32> {
    self.activity_graph_hash.as_ref()
  }

  pub fn reset_activity_graph_hash(&mut self) {
    self.activity_graph_hash = None;
  }

}



