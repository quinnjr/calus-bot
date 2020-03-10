/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsVendorsDestinyVendorGroupComponent : This component returns references to all of the Vendors in the response, grouped by categorizations that Bungie has deemed to be interesting, in the order in which both the groups and the vendors within that group should be rendered.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsVendorsDestinyVendorGroupComponent {
  /// The ordered list of groups being returned.
  #[serde(rename = "groups")]
  groups: Option<Vec<::models::DestinyComponentsVendorsDestinyVendorGroup>>
}

impl DestinyComponentsVendorsDestinyVendorGroupComponent {
  /// This component returns references to all of the Vendors in the response, grouped by categorizations that Bungie has deemed to be interesting, in the order in which both the groups and the vendors within that group should be rendered.
  pub fn new() -> DestinyComponentsVendorsDestinyVendorGroupComponent {
    DestinyComponentsVendorsDestinyVendorGroupComponent {
      groups: None
    }
  }

  pub fn set_groups(&mut self, groups: Vec<::models::DestinyComponentsVendorsDestinyVendorGroup>) {
    self.groups = Some(groups);
  }

  pub fn with_groups(mut self, groups: Vec<::models::DestinyComponentsVendorsDestinyVendorGroup>) -> DestinyComponentsVendorsDestinyVendorGroupComponent {
    self.groups = Some(groups);
    self
  }

  pub fn groups(&self) -> Option<&Vec<::models::DestinyComponentsVendorsDestinyVendorGroup>> {
    self.groups.as_ref()
  }

  pub fn reset_groups(&mut self) {
    self.groups = None;
  }

}



