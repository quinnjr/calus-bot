/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsVendorsDestinyVendorBaseComponent : This component contains essential/summary information about the vendor.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsVendorsDestinyVendorBaseComponent {
  /// The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition.
  #[serde(rename = "vendorHash")]
  vendor_hash: Option<i32>,
  /// The date when this vendor's inventory will next rotate/refresh.  Note that this is distinct from the date ranges that the vendor is visible/available in-game: this field indicates the specific time when the vendor's available items refresh and rotate, regardless of whether the vendor is actually available at that time. Unfortunately, these two values may be (and are, for the case of important vendors like Xur) different.  Issue https://github.com/Bungie-net/api/issues/353 is tracking a fix to start providing visibility date ranges where possible in addition to this refresh date, so that all important dates for vendors are available for use.
  #[serde(rename = "nextRefreshDate")]
  next_refresh_date: Option<String>,
  /// If True, the Vendor is currently accessible.   If False, they may not actually be visible in the world at the moment.
  #[serde(rename = "enabled")]
  enabled: Option<bool>
}

impl DestinyComponentsVendorsDestinyVendorBaseComponent {
  /// This component contains essential/summary information about the vendor.
  pub fn new() -> DestinyComponentsVendorsDestinyVendorBaseComponent {
    DestinyComponentsVendorsDestinyVendorBaseComponent {
      vendor_hash: None,
      next_refresh_date: None,
      enabled: None
    }
  }

  pub fn set_vendor_hash(&mut self, vendor_hash: i32) {
    self.vendor_hash = Some(vendor_hash);
  }

  pub fn with_vendor_hash(mut self, vendor_hash: i32) -> DestinyComponentsVendorsDestinyVendorBaseComponent {
    self.vendor_hash = Some(vendor_hash);
    self
  }

  pub fn vendor_hash(&self) -> Option<&i32> {
    self.vendor_hash.as_ref()
  }

  pub fn reset_vendor_hash(&mut self) {
    self.vendor_hash = None;
  }

  pub fn set_next_refresh_date(&mut self, next_refresh_date: String) {
    self.next_refresh_date = Some(next_refresh_date);
  }

  pub fn with_next_refresh_date(mut self, next_refresh_date: String) -> DestinyComponentsVendorsDestinyVendorBaseComponent {
    self.next_refresh_date = Some(next_refresh_date);
    self
  }

  pub fn next_refresh_date(&self) -> Option<&String> {
    self.next_refresh_date.as_ref()
  }

  pub fn reset_next_refresh_date(&mut self) {
    self.next_refresh_date = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> DestinyComponentsVendorsDestinyVendorBaseComponent {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

}



