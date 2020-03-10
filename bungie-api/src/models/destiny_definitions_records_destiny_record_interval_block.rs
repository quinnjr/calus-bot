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
pub struct DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
  #[serde(rename = "intervalObjectives")]
  interval_objectives: Option<Vec<::models::DestinyDefinitionsRecordsDestinyRecordIntervalObjective>>,
  #[serde(rename = "originalObjectiveArrayInsertionIndex")]
  original_objective_array_insertion_index: Option<i32>
}

impl DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
  pub fn new() -> DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
    DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
      interval_objectives: None,
      original_objective_array_insertion_index: None
    }
  }

  pub fn set_interval_objectives(&mut self, interval_objectives: Vec<::models::DestinyDefinitionsRecordsDestinyRecordIntervalObjective>) {
    self.interval_objectives = Some(interval_objectives);
  }

  pub fn with_interval_objectives(mut self, interval_objectives: Vec<::models::DestinyDefinitionsRecordsDestinyRecordIntervalObjective>) -> DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
    self.interval_objectives = Some(interval_objectives);
    self
  }

  pub fn interval_objectives(&self) -> Option<&Vec<::models::DestinyDefinitionsRecordsDestinyRecordIntervalObjective>> {
    self.interval_objectives.as_ref()
  }

  pub fn reset_interval_objectives(&mut self) {
    self.interval_objectives = None;
  }

  pub fn set_original_objective_array_insertion_index(&mut self, original_objective_array_insertion_index: i32) {
    self.original_objective_array_insertion_index = Some(original_objective_array_insertion_index);
  }

  pub fn with_original_objective_array_insertion_index(mut self, original_objective_array_insertion_index: i32) -> DestinyDefinitionsRecordsDestinyRecordIntervalBlock {
    self.original_objective_array_insertion_index = Some(original_objective_array_insertion_index);
    self
  }

  pub fn original_objective_array_insertion_index(&self) -> Option<&i32> {
    self.original_objective_array_insertion_index.as_ref()
  }

  pub fn reset_original_objective_array_insertion_index(&mut self) {
    self.original_objective_array_insertion_index = None;
  }

}


