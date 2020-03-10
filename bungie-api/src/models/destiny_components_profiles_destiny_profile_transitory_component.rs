/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsProfilesDestinyProfileTransitoryComponent : This is an experimental set of data that Bungie considers to be \"transitory\" - information that may be useful for API users, but that is coming from a non-authoritative data source about information that could potentially change at a more frequent pace than Bungie.net will receive updates about it.  This information is provided exclusively for convenience should any of it be useful to users: we provide no guarantees to the accuracy or timeliness of data that comes from this source. Know that this data can potentially be out-of-date or even wrong entirely if the user disconnected from the game or suddenly changed their status before we can receive refreshed data.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsProfilesDestinyProfileTransitoryComponent {
  /// If you have any members currently in your party, this is some (very) bare-bones information about those members.
  #[serde(rename = "partyMembers")]
  party_members: Option<Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryPartyMember>>,
  /// If you are in an activity, this is some transitory info about the activity currently being played.
  #[serde(rename = "currentActivity")]
  current_activity: Option<Value>,
  /// Information about whether and what might prevent you from joining this person on a fireteam.
  #[serde(rename = "joinability")]
  joinability: Option<Value>,
  /// Information about tracked entities.
  #[serde(rename = "tracking")]
  tracking: Option<Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry>>,
  /// The hash identifier for the DestinyDestinationDefinition of the last location you were orbiting when in orbit.
  #[serde(rename = "lastOrbitedDestinationHash")]
  last_orbited_destination_hash: Option<i32>
}

impl DestinyComponentsProfilesDestinyProfileTransitoryComponent {
  /// This is an experimental set of data that Bungie considers to be \"transitory\" - information that may be useful for API users, but that is coming from a non-authoritative data source about information that could potentially change at a more frequent pace than Bungie.net will receive updates about it.  This information is provided exclusively for convenience should any of it be useful to users: we provide no guarantees to the accuracy or timeliness of data that comes from this source. Know that this data can potentially be out-of-date or even wrong entirely if the user disconnected from the game or suddenly changed their status before we can receive refreshed data.
  pub fn new() -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    DestinyComponentsProfilesDestinyProfileTransitoryComponent {
      party_members: None,
      current_activity: None,
      joinability: None,
      tracking: None,
      last_orbited_destination_hash: None
    }
  }

  pub fn set_party_members(&mut self, party_members: Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryPartyMember>) {
    self.party_members = Some(party_members);
  }

  pub fn with_party_members(mut self, party_members: Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryPartyMember>) -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    self.party_members = Some(party_members);
    self
  }

  pub fn party_members(&self) -> Option<&Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryPartyMember>> {
    self.party_members.as_ref()
  }

  pub fn reset_party_members(&mut self) {
    self.party_members = None;
  }

  pub fn set_current_activity(&mut self, current_activity: Value) {
    self.current_activity = Some(current_activity);
  }

  pub fn with_current_activity(mut self, current_activity: Value) -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    self.current_activity = Some(current_activity);
    self
  }

  pub fn current_activity(&self) -> Option<&Value> {
    self.current_activity.as_ref()
  }

  pub fn reset_current_activity(&mut self) {
    self.current_activity = None;
  }

  pub fn set_joinability(&mut self, joinability: Value) {
    self.joinability = Some(joinability);
  }

  pub fn with_joinability(mut self, joinability: Value) -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    self.joinability = Some(joinability);
    self
  }

  pub fn joinability(&self) -> Option<&Value> {
    self.joinability.as_ref()
  }

  pub fn reset_joinability(&mut self) {
    self.joinability = None;
  }

  pub fn set_tracking(&mut self, tracking: Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry>) {
    self.tracking = Some(tracking);
  }

  pub fn with_tracking(mut self, tracking: Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry>) -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    self.tracking = Some(tracking);
    self
  }

  pub fn tracking(&self) -> Option<&Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry>> {
    self.tracking.as_ref()
  }

  pub fn reset_tracking(&mut self) {
    self.tracking = None;
  }

  pub fn set_last_orbited_destination_hash(&mut self, last_orbited_destination_hash: i32) {
    self.last_orbited_destination_hash = Some(last_orbited_destination_hash);
  }

  pub fn with_last_orbited_destination_hash(mut self, last_orbited_destination_hash: i32) -> DestinyComponentsProfilesDestinyProfileTransitoryComponent {
    self.last_orbited_destination_hash = Some(last_orbited_destination_hash);
    self
  }

  pub fn last_orbited_destination_hash(&self) -> Option<&i32> {
    self.last_orbited_destination_hash.as_ref()
  }

  pub fn reset_last_orbited_destination_hash(&mut self) {
    self.last_orbited_destination_hash = None;
  }

}


