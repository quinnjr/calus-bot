/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyRewardSourceCategory : BNet's custom categorization of reward sources. We took a look at the existing ways that items could be spawned, and tried to make high-level categorizations of them. This needs to be re-evaluated for Destiny 2.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyRewardSourceCategory {
}

impl DestinyDefinitionsDestinyRewardSourceCategory {
  /// BNet's custom categorization of reward sources. We took a look at the existing ways that items could be spawned, and tried to make high-level categorizations of them. This needs to be re-evaluated for Destiny 2.
  pub fn new() -> DestinyDefinitionsDestinyRewardSourceCategory {
    DestinyDefinitionsDestinyRewardSourceCategory {
    }
  }

}

// TODO enum 
// List of Destiny.Definitions.DestinyRewardSourceCategory
//const (
//  
//  
//  
//)


