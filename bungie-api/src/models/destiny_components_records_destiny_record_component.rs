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
pub struct DestinyComponentsRecordsDestinyRecordComponent {
  #[serde(rename = "state")]
  state: Option<i32>,
  #[serde(rename = "objectives")]
  objectives: Option<Vec<::models::DestinyQuestsDestinyObjectiveProgress>>,
  #[serde(rename = "intervalObjectives")]
  interval_objectives: Option<Vec<::models::DestinyQuestsDestinyObjectiveProgress>>,
  #[serde(rename = "intervalsRedeemedCount")]
  intervals_redeemed_count: Option<i32>
}

impl DestinyComponentsRecordsDestinyRecordComponent {
  pub fn new() -> DestinyComponentsRecordsDestinyRecordComponent {
    DestinyComponentsRecordsDestinyRecordComponent {
      state: None,
      objectives: None,
      interval_objectives: None,
      intervals_redeemed_count: None
    }
  }

  pub fn set_state(&mut self, state: i32) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: i32) -> DestinyComponentsRecordsDestinyRecordComponent {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&i32> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_objectives(&mut self, objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) {
    self.objectives = Some(objectives);
  }

  pub fn with_objectives(mut self, objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) -> DestinyComponentsRecordsDestinyRecordComponent {
    self.objectives = Some(objectives);
    self
  }

  pub fn objectives(&self) -> Option<&Vec<::models::DestinyQuestsDestinyObjectiveProgress>> {
    self.objectives.as_ref()
  }

  pub fn reset_objectives(&mut self) {
    self.objectives = None;
  }

  pub fn set_interval_objectives(&mut self, interval_objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) {
    self.interval_objectives = Some(interval_objectives);
  }

  pub fn with_interval_objectives(mut self, interval_objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) -> DestinyComponentsRecordsDestinyRecordComponent {
    self.interval_objectives = Some(interval_objectives);
    self
  }

  pub fn interval_objectives(&self) -> Option<&Vec<::models::DestinyQuestsDestinyObjectiveProgress>> {
    self.interval_objectives.as_ref()
  }

  pub fn reset_interval_objectives(&mut self) {
    self.interval_objectives = None;
  }

  pub fn set_intervals_redeemed_count(&mut self, intervals_redeemed_count: i32) {
    self.intervals_redeemed_count = Some(intervals_redeemed_count);
  }

  pub fn with_intervals_redeemed_count(mut self, intervals_redeemed_count: i32) -> DestinyComponentsRecordsDestinyRecordComponent {
    self.intervals_redeemed_count = Some(intervals_redeemed_count);
    self
  }

  pub fn intervals_redeemed_count(&self) -> Option<&i32> {
    self.intervals_redeemed_count.as_ref()
  }

  pub fn reset_intervals_redeemed_count(&mut self) {
    self.intervals_redeemed_count = None;
  }

}



