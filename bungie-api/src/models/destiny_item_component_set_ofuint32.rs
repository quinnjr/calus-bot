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
pub struct DestinyItemComponentSetOfuint32 {
  #[serde(rename = "instances")]
  instances: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent>,
  #[serde(rename = "perks")]
  perks: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent>,
  #[serde(rename = "renderData")]
  render_data: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent>,
  #[serde(rename = "stats")]
  stats: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent>,
  #[serde(rename = "sockets")]
  sockets: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent>,
  #[serde(rename = "reusablePlugs")]
  reusable_plugs: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent>,
  #[serde(rename = "plugObjectives")]
  plug_objectives: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent>,
  #[serde(rename = "talentGrids")]
  talent_grids: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent>,
  #[serde(rename = "plugStates")]
  plug_states: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>,
  #[serde(rename = "objectives")]
  objectives: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent>
}

impl DestinyItemComponentSetOfuint32 {
  pub fn new() -> DestinyItemComponentSetOfuint32 {
    DestinyItemComponentSetOfuint32 {
      instances: None,
      perks: None,
      render_data: None,
      stats: None,
      sockets: None,
      reusable_plugs: None,
      plug_objectives: None,
      talent_grids: None,
      plug_states: None,
      objectives: None
    }
  }

  pub fn set_instances(&mut self, instances: ::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent) {
    self.instances = Some(instances);
  }

  pub fn with_instances(mut self, instances: ::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent) -> DestinyItemComponentSetOfuint32 {
    self.instances = Some(instances);
    self
  }

  pub fn instances(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent> {
    self.instances.as_ref()
  }

  pub fn reset_instances(&mut self) {
    self.instances = None;
  }

  pub fn set_perks(&mut self, perks: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent) {
    self.perks = Some(perks);
  }

  pub fn with_perks(mut self, perks: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent) -> DestinyItemComponentSetOfuint32 {
    self.perks = Some(perks);
    self
  }

  pub fn perks(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent> {
    self.perks.as_ref()
  }

  pub fn reset_perks(&mut self) {
    self.perks = None;
  }

  pub fn set_render_data(&mut self, render_data: ::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent) {
    self.render_data = Some(render_data);
  }

  pub fn with_render_data(mut self, render_data: ::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent) -> DestinyItemComponentSetOfuint32 {
    self.render_data = Some(render_data);
    self
  }

  pub fn render_data(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent> {
    self.render_data.as_ref()
  }

  pub fn reset_render_data(&mut self) {
    self.render_data = None;
  }

  pub fn set_stats(&mut self, stats: ::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent) {
    self.stats = Some(stats);
  }

  pub fn with_stats(mut self, stats: ::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent) -> DestinyItemComponentSetOfuint32 {
    self.stats = Some(stats);
    self
  }

  pub fn stats(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent> {
    self.stats.as_ref()
  }

  pub fn reset_stats(&mut self) {
    self.stats = None;
  }

  pub fn set_sockets(&mut self, sockets: ::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent) {
    self.sockets = Some(sockets);
  }

  pub fn with_sockets(mut self, sockets: ::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent) -> DestinyItemComponentSetOfuint32 {
    self.sockets = Some(sockets);
    self
  }

  pub fn sockets(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent> {
    self.sockets.as_ref()
  }

  pub fn reset_sockets(&mut self) {
    self.sockets = None;
  }

  pub fn set_reusable_plugs(&mut self, reusable_plugs: ::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent) {
    self.reusable_plugs = Some(reusable_plugs);
  }

  pub fn with_reusable_plugs(mut self, reusable_plugs: ::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent) -> DestinyItemComponentSetOfuint32 {
    self.reusable_plugs = Some(reusable_plugs);
    self
  }

  pub fn reusable_plugs(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent> {
    self.reusable_plugs.as_ref()
  }

  pub fn reset_reusable_plugs(&mut self) {
    self.reusable_plugs = None;
  }

  pub fn set_plug_objectives(&mut self, plug_objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent) {
    self.plug_objectives = Some(plug_objectives);
  }

  pub fn with_plug_objectives(mut self, plug_objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent) -> DestinyItemComponentSetOfuint32 {
    self.plug_objectives = Some(plug_objectives);
    self
  }

  pub fn plug_objectives(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent> {
    self.plug_objectives.as_ref()
  }

  pub fn reset_plug_objectives(&mut self) {
    self.plug_objectives = None;
  }

  pub fn set_talent_grids(&mut self, talent_grids: ::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent) {
    self.talent_grids = Some(talent_grids);
  }

  pub fn with_talent_grids(mut self, talent_grids: ::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent) -> DestinyItemComponentSetOfuint32 {
    self.talent_grids = Some(talent_grids);
    self
  }

  pub fn talent_grids(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent> {
    self.talent_grids.as_ref()
  }

  pub fn reset_talent_grids(&mut self) {
    self.talent_grids = None;
  }

  pub fn set_plug_states(&mut self, plug_states: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent) {
    self.plug_states = Some(plug_states);
  }

  pub fn with_plug_states(mut self, plug_states: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent) -> DestinyItemComponentSetOfuint32 {
    self.plug_states = Some(plug_states);
    self
  }

  pub fn plug_states(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent> {
    self.plug_states.as_ref()
  }

  pub fn reset_plug_states(&mut self) {
    self.plug_states = None;
  }

  pub fn set_objectives(&mut self, objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent) {
    self.objectives = Some(objectives);
  }

  pub fn with_objectives(mut self, objectives: ::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent) -> DestinyItemComponentSetOfuint32 {
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



