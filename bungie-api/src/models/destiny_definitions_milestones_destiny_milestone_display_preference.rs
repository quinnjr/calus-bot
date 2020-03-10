/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsMilestonesDestinyMilestoneDisplayPreference : A hint for the UI as to what display information ought to be shown. Defaults to showing the static MilestoneDefinition's display properties.   If for some reason the indicated property is not populated, fall back to the MilestoneDefinition.displayProperties.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsMilestonesDestinyMilestoneDisplayPreference {
}

impl DestinyDefinitionsMilestonesDestinyMilestoneDisplayPreference {
  /// A hint for the UI as to what display information ought to be shown. Defaults to showing the static MilestoneDefinition's display properties.   If for some reason the indicated property is not populated, fall back to the MilestoneDefinition.displayProperties.
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneDisplayPreference {
    DestinyDefinitionsMilestonesDestinyMilestoneDisplayPreference {
    }
  }

}

// TODO enum 
// List of Destiny.Definitions.Milestones.DestinyMilestoneDisplayPreference
//const (
//  
//  
//  
//)


