/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemTranslationBlockDefinition : This Block defines the rendering data associated with the item, if any.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemTranslationBlockDefinition {
  #[serde(rename = "weaponPatternIdentifier")]
  weapon_pattern_identifier: Option<String>,
  #[serde(rename = "weaponPatternHash")]
  weapon_pattern_hash: Option<i32>,
  #[serde(rename = "defaultDyes")]
  default_dyes: Option<Vec<::models::DestinyDyeReference>>,
  #[serde(rename = "lockedDyes")]
  locked_dyes: Option<Vec<::models::DestinyDyeReference>>,
  #[serde(rename = "customDyes")]
  custom_dyes: Option<Vec<::models::DestinyDyeReference>>,
  #[serde(rename = "arrangements")]
  arrangements: Option<Vec<::models::DestinyDefinitionsDestinyGearArtArrangementReference>>,
  #[serde(rename = "hasGeometry")]
  has_geometry: Option<bool>
}

impl DestinyDefinitionsDestinyItemTranslationBlockDefinition {
  /// This Block defines the rendering data associated with the item, if any.
  pub fn new() -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    DestinyDefinitionsDestinyItemTranslationBlockDefinition {
      weapon_pattern_identifier: None,
      weapon_pattern_hash: None,
      default_dyes: None,
      locked_dyes: None,
      custom_dyes: None,
      arrangements: None,
      has_geometry: None
    }
  }

  pub fn set_weapon_pattern_identifier(&mut self, weapon_pattern_identifier: String) {
    self.weapon_pattern_identifier = Some(weapon_pattern_identifier);
  }

  pub fn with_weapon_pattern_identifier(mut self, weapon_pattern_identifier: String) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.weapon_pattern_identifier = Some(weapon_pattern_identifier);
    self
  }

  pub fn weapon_pattern_identifier(&self) -> Option<&String> {
    self.weapon_pattern_identifier.as_ref()
  }

  pub fn reset_weapon_pattern_identifier(&mut self) {
    self.weapon_pattern_identifier = None;
  }

  pub fn set_weapon_pattern_hash(&mut self, weapon_pattern_hash: i32) {
    self.weapon_pattern_hash = Some(weapon_pattern_hash);
  }

  pub fn with_weapon_pattern_hash(mut self, weapon_pattern_hash: i32) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.weapon_pattern_hash = Some(weapon_pattern_hash);
    self
  }

  pub fn weapon_pattern_hash(&self) -> Option<&i32> {
    self.weapon_pattern_hash.as_ref()
  }

  pub fn reset_weapon_pattern_hash(&mut self) {
    self.weapon_pattern_hash = None;
  }

  pub fn set_default_dyes(&mut self, default_dyes: Vec<::models::DestinyDyeReference>) {
    self.default_dyes = Some(default_dyes);
  }

  pub fn with_default_dyes(mut self, default_dyes: Vec<::models::DestinyDyeReference>) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.default_dyes = Some(default_dyes);
    self
  }

  pub fn default_dyes(&self) -> Option<&Vec<::models::DestinyDyeReference>> {
    self.default_dyes.as_ref()
  }

  pub fn reset_default_dyes(&mut self) {
    self.default_dyes = None;
  }

  pub fn set_locked_dyes(&mut self, locked_dyes: Vec<::models::DestinyDyeReference>) {
    self.locked_dyes = Some(locked_dyes);
  }

  pub fn with_locked_dyes(mut self, locked_dyes: Vec<::models::DestinyDyeReference>) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.locked_dyes = Some(locked_dyes);
    self
  }

  pub fn locked_dyes(&self) -> Option<&Vec<::models::DestinyDyeReference>> {
    self.locked_dyes.as_ref()
  }

  pub fn reset_locked_dyes(&mut self) {
    self.locked_dyes = None;
  }

  pub fn set_custom_dyes(&mut self, custom_dyes: Vec<::models::DestinyDyeReference>) {
    self.custom_dyes = Some(custom_dyes);
  }

  pub fn with_custom_dyes(mut self, custom_dyes: Vec<::models::DestinyDyeReference>) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.custom_dyes = Some(custom_dyes);
    self
  }

  pub fn custom_dyes(&self) -> Option<&Vec<::models::DestinyDyeReference>> {
    self.custom_dyes.as_ref()
  }

  pub fn reset_custom_dyes(&mut self) {
    self.custom_dyes = None;
  }

  pub fn set_arrangements(&mut self, arrangements: Vec<::models::DestinyDefinitionsDestinyGearArtArrangementReference>) {
    self.arrangements = Some(arrangements);
  }

  pub fn with_arrangements(mut self, arrangements: Vec<::models::DestinyDefinitionsDestinyGearArtArrangementReference>) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.arrangements = Some(arrangements);
    self
  }

  pub fn arrangements(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyGearArtArrangementReference>> {
    self.arrangements.as_ref()
  }

  pub fn reset_arrangements(&mut self) {
    self.arrangements = None;
  }

  pub fn set_has_geometry(&mut self, has_geometry: bool) {
    self.has_geometry = Some(has_geometry);
  }

  pub fn with_has_geometry(mut self, has_geometry: bool) -> DestinyDefinitionsDestinyItemTranslationBlockDefinition {
    self.has_geometry = Some(has_geometry);
    self
  }

  pub fn has_geometry(&self) -> Option<&bool> {
    self.has_geometry.as_ref()
  }

  pub fn reset_has_geometry(&mut self) {
    self.has_geometry = None;
  }

}


