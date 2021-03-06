/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsProfilesDestinyProfileTransitoryPartyMember : This is some bare minimum information about a party member in a Fireteam. Unfortunately, without great computational expense on our side we can only get at the data contained here. I'd like to give you a character ID for example, but we don't have it. But we do have these three pieces of information. May they help you on your quest to show meaningful data about current Fireteams.  Notably, we don't and can't feasibly return info on characters. If you can, try to use just the data below for your UI and purposes. Only hit us with further queries if you absolutely must know the character ID of the currently playing character. Pretty please with sugar on top.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
  /// The Membership ID that matches the party member.
  #[serde(rename = "membershipId")]
  membership_id: Option<i64>,
  /// The identifier for the DestinyInventoryItemDefinition of the player's emblem.
  #[serde(rename = "emblemHash")]
  emblem_hash: Option<i32>,
  /// The player's last known display name.
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  /// A Flags Enumeration value indicating the states that the player is in relevant to being on a fireteam.
  #[serde(rename = "status")]
  status: Option<i32>
}

impl DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
  /// This is some bare minimum information about a party member in a Fireteam. Unfortunately, without great computational expense on our side we can only get at the data contained here. I'd like to give you a character ID for example, but we don't have it. But we do have these three pieces of information. May they help you on your quest to show meaningful data about current Fireteams.  Notably, we don't and can't feasibly return info on characters. If you can, try to use just the data below for your UI and purposes. Only hit us with further queries if you absolutely must know the character ID of the currently playing character. Pretty please with sugar on top.
  pub fn new() -> DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
    DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
      membership_id: None,
      emblem_hash: None,
      display_name: None,
      status: None
    }
  }

  pub fn set_membership_id(&mut self, membership_id: i64) {
    self.membership_id = Some(membership_id);
  }

  pub fn with_membership_id(mut self, membership_id: i64) -> DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
    self.membership_id = Some(membership_id);
    self
  }

  pub fn membership_id(&self) -> Option<&i64> {
    self.membership_id.as_ref()
  }

  pub fn reset_membership_id(&mut self) {
    self.membership_id = None;
  }

  pub fn set_emblem_hash(&mut self, emblem_hash: i32) {
    self.emblem_hash = Some(emblem_hash);
  }

  pub fn with_emblem_hash(mut self, emblem_hash: i32) -> DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
    self.emblem_hash = Some(emblem_hash);
    self
  }

  pub fn emblem_hash(&self) -> Option<&i32> {
    self.emblem_hash.as_ref()
  }

  pub fn reset_emblem_hash(&mut self) {
    self.emblem_hash = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_status(&mut self, status: i32) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: i32) -> DestinyComponentsProfilesDestinyProfileTransitoryPartyMember {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&i32> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



