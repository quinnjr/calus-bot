/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyResponsesDestinyPublicVendorsResponse : A response containing all valid components for the public Vendors endpoint.   It is a decisively smaller subset of data compared to what we can get when we know the specific user making the request.   If you want any of the other data - item details, whether or not you can buy it, etc... you'll have to call in the context of a character. I know, sad but true.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyPublicVendorsResponse {
  /// For Vendors being returned, this will give you the information you need to group them and order them in the same way that the Bungie Companion app performs grouping. It will automatically be returned if you request the Vendors component.  COMPONENT TYPE: Vendors
  #[serde(rename = "vendorGroups")]
  vendor_groups: Option<Value>,
  /// The base properties of the vendor. These are keyed by the Vendor Hash, so you will get one Vendor Component per vendor returned.  COMPONENT TYPE: Vendors
  #[serde(rename = "vendors")]
  vendors: Option<Value>,
  /// Categories that the vendor has available, and references to the sales therein. These are keyed by the Vendor Hash, so you will get one Categories Component per vendor returned.  COMPONENT TYPE: VendorCategories
  #[serde(rename = "categories")]
  categories: Option<Value>,
  /// Sales, keyed by the vendorItemIndex of the item being sold. These are keyed by the Vendor Hash, so you will get one Sale Item Set Component per vendor returned.  Note that within the Sale Item Set component, the sales are themselves keyed by the vendorSaleIndex, so you can relate it to the corrent sale item definition within the Vendor's definition.  COMPONENT TYPE: VendorSales
  #[serde(rename = "sales")]
  sales: Option<Value>
}

impl DestinyResponsesDestinyPublicVendorsResponse {
  /// A response containing all valid components for the public Vendors endpoint.   It is a decisively smaller subset of data compared to what we can get when we know the specific user making the request.   If you want any of the other data - item details, whether or not you can buy it, etc... you'll have to call in the context of a character. I know, sad but true.
  pub fn new() -> DestinyResponsesDestinyPublicVendorsResponse {
    DestinyResponsesDestinyPublicVendorsResponse {
      vendor_groups: None,
      vendors: None,
      categories: None,
      sales: None
    }
  }

  pub fn set_vendor_groups(&mut self, vendor_groups: Value) {
    self.vendor_groups = Some(vendor_groups);
  }

  pub fn with_vendor_groups(mut self, vendor_groups: Value) -> DestinyResponsesDestinyPublicVendorsResponse {
    self.vendor_groups = Some(vendor_groups);
    self
  }

  pub fn vendor_groups(&self) -> Option<&Value> {
    self.vendor_groups.as_ref()
  }

  pub fn reset_vendor_groups(&mut self) {
    self.vendor_groups = None;
  }

  pub fn set_vendors(&mut self, vendors: Value) {
    self.vendors = Some(vendors);
  }

  pub fn with_vendors(mut self, vendors: Value) -> DestinyResponsesDestinyPublicVendorsResponse {
    self.vendors = Some(vendors);
    self
  }

  pub fn vendors(&self) -> Option<&Value> {
    self.vendors.as_ref()
  }

  pub fn reset_vendors(&mut self) {
    self.vendors = None;
  }

  pub fn set_categories(&mut self, categories: Value) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Value) -> DestinyResponsesDestinyPublicVendorsResponse {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Value> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

  pub fn set_sales(&mut self, sales: Value) {
    self.sales = Some(sales);
  }

  pub fn with_sales(mut self, sales: Value) -> DestinyResponsesDestinyPublicVendorsResponse {
    self.sales = Some(sales);
    self
  }

  pub fn sales(&self) -> Option<&Value> {
    self.sales.as_ref()
  }

  pub fn reset_sales(&mut self) {
    self.sales = None;
  }

}



