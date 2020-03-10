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
pub struct DestinyBaseItemComponentSetOfint64 {
  #[serde(rename = "objectives")]
  objectives: Option<::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent>
}

impl DestinyBaseItemComponentSetOfint64 {
  pub fn new() -> DestinyBaseItemComponentSetOfint64 {
    DestinyBaseItemComponentSetOfint64 {
      objectives: None
    }
  }

  pub fn set_objectives(&mut self, objectives: ::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent) {
    self.objectives = Some(objectives);
  }

  pub fn with_objectives(mut self, objectives: ::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent) -> DestinyBaseItemComponentSetOfint64 {
    self.objectives = Some(objectives);
    self
  }

  pub fn objectives(&self) -> Option<&::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent> {
    self.objectives.as_ref()
  }

  pub fn reset_objectives(&mut self) {
    self.objectives = None;
  }

}


