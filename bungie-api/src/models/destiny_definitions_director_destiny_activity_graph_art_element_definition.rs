/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition : These Art Elements are meant to represent one-off visual effects overlaid on the map. Currently, we do not have a pipeline to import the assets for these overlays, so this info exists as a placeholder for when such a pipeline exists (if it ever will)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition {
  /// The position on the map of the art element.
  #[serde(rename = "position")]
  position: Option<Value>
}

impl DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition {
  /// These Art Elements are meant to represent one-off visual effects overlaid on the map. Currently, we do not have a pipeline to import the assets for these overlays, so this info exists as a placeholder for when such a pipeline exists (if it ever will)
  pub fn new() -> DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition {
    DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition {
      position: None
    }
  }

  pub fn set_position(&mut self, position: Value) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: Value) -> DestinyDefinitionsDirectorDestinyActivityGraphArtElementDefinition {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&Value> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

}


