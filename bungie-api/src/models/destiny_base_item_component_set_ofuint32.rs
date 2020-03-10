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
pub struct DestinyBaseItemComponentSetOfuint32 {
  #[serde(rename = "objectives")]
  objectives: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent>
}

impl DestinyBaseItemComponentSetOfuint32 {
  pub fn new() -> DestinyBaseItemComponentSetOfuint32 {
    DestinyBaseItemComponentSetOfuint32 {
      objectives: None
    }
  }

  pub fn set_objectives(&mut self, objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent) {
    self.objectives = Some(objectives);
  }

  pub fn with_objectives(mut self, objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent) -> DestinyBaseItemComponentSetOfuint32 {
    self.objectives = Some(objectives);
    self
  }

  pub fn objectives(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent> {
    self.objectives.as_ref()
  }

  pub fn reset_objectives(&mut self) {
    self.objectives = None;
  }

}


