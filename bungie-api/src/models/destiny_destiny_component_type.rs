/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDestinyComponentType : Represents the possible components that can be returned from Destiny \"Get\" calls such as GetProfile, GetCharacter, GetVendor etc...  When making one of these requests, you will pass one or more of these components as a comma separated list in the \"?components=\" querystring parameter. For instance, if you want baseline Profile data, Character Data, and character progressions, you would pass \"?components=Profiles,Characters,CharacterProgressions\" You may use either the numerical or string values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDestinyComponentType {
}

impl DestinyDestinyComponentType {
  /// Represents the possible components that can be returned from Destiny \"Get\" calls such as GetProfile, GetCharacter, GetVendor etc...  When making one of these requests, you will pass one or more of these components as a comma separated list in the \"?components=\" querystring parameter. For instance, if you want baseline Profile data, Character Data, and character progressions, you would pass \"?components=Profiles,Characters,CharacterProgressions\" You may use either the numerical or string values.
  pub fn new() -> DestinyDestinyComponentType {
    DestinyDestinyComponentType {
    }
  }

}

// TODO enum 
// List of Destiny.DestinyComponentType
//const (
//  
//  
//  
//)


