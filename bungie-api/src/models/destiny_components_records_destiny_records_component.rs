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
pub struct DestinyComponentsRecordsDestinyRecordsComponent {
  #[serde(rename = "records")]
  records: Option<::std::collections::HashMap<String, ::models::DestinyComponentsRecordsDestinyRecordComponent>>
}

impl DestinyComponentsRecordsDestinyRecordsComponent {
  pub fn new() -> DestinyComponentsRecordsDestinyRecordsComponent {
    DestinyComponentsRecordsDestinyRecordsComponent {
      records: None
    }
  }

  pub fn set_records(&mut self, records: ::std::collections::HashMap<String, ::models::DestinyComponentsRecordsDestinyRecordComponent>) {
    self.records = Some(records);
  }

  pub fn with_records(mut self, records: ::std::collections::HashMap<String, ::models::DestinyComponentsRecordsDestinyRecordComponent>) -> DestinyComponentsRecordsDestinyRecordsComponent {
    self.records = Some(records);
    self
  }

  pub fn records(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyComponentsRecordsDestinyRecordComponent>> {
    self.records.as_ref()
  }

  pub fn reset_records(&mut self) {
    self.records = None;
  }

}


