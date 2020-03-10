/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinySpecialItemType : As you run into items that need to be classified for Milestone purposes in ways that we cannot infer via direct data, add a new classification here and use a string constant to represent it in the local item config file.  NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinySpecialItemType {
}

impl DestinySpecialItemType {
  /// As you run into items that need to be classified for Milestone purposes in ways that we cannot infer via direct data, add a new classification here and use a string constant to represent it in the local item config file.  NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
  pub fn new() -> DestinySpecialItemType {
    DestinySpecialItemType {
    }
  }

}

// TODO enum 
// List of Destiny.SpecialItemType
//const (
//  
//  
//  
//)


