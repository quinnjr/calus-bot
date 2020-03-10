/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesVendorsDestinyVendorSaleItemComponent : Request this component if you want the details about an item being sold in relation to the character making the request: whether the character can buy it, whether they can afford it, and other data related to purchasing the item.  Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's \"items\" property.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
  /// A flag indicating whether the requesting character can buy the item, and if not the reasons why the character can't buy it.
  #[serde(rename = "saleStatus")]
  sale_status: Option<i32>,
  /// If you can't buy the item due to a complex character state, these will be hashes for DestinyUnlockDefinitions that you can check to see messages regarding the failure (if the unlocks have human readable information: it is not guaranteed that Unlocks will have human readable strings, and your application will have to handle that)  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
  #[serde(rename = "requiredUnlocks")]
  required_unlocks: Option<Vec<i32>>,
  /// If any complex unlock states are checked in determining purchasability, these will be returned here along with the status of the unlock check.  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
  #[serde(rename = "unlockStatuses")]
  unlock_statuses: Option<Vec<::models::DestinyDestinyUnlockStatus>>,
  /// Indexes in to the \"failureStrings\" lookup table in DestinyVendorDefinition for the given Vendor. Gives some more reliable failure information for why you can't purchase an item.  It is preferred to use these over requiredUnlocks and unlockStatuses: the latter are provided mostly in case someone can do something interesting with it that I didn't anticipate.
  #[serde(rename = "failureIndexes")]
  failure_indexes: Option<Vec<i32>>,
  /// A flags enumeration value representing the current state of any \"state modifiers\" on the item being sold. These are meant to correspond with some sort of visual indicator as to the augmentation: for instance, if an item is on sale or if you already own the item in question.  Determining how you want to represent these in your own app (or if you even want to) is an exercise left for the reader.
  #[serde(rename = "augments")]
  augments: Option<i32>,
  /// The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch.   Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment.
  #[serde(rename = "vendorItemIndex")]
  vendor_item_index: Option<i32>,
  /// The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item.
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  /// If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.  If you don't do this, certain items whose styles are being overridden by socketed items - such as the \"Recycle Shader\" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate.
  #[serde(rename = "overrideStyleItemHash")]
  override_style_item_hash: Option<i32>,
  /// How much of the item you'll be getting.
  #[serde(rename = "quantity")]
  quantity: Option<i32>,
  /// A summary of the current costs of the item.
  #[serde(rename = "costs")]
  costs: Option<Vec<::models::DestinyDestinyItemQuantity>>,
  /// If this item has its own custom date where it may be removed from the Vendor's rotation, this is that date.  Note that there's not actually any guarantee that it will go away: it could be chosen again and end up still being in the Vendor's sale items! But this is the next date where that test will occur, and is also the date that the game shows for availability on things like Bounties being sold. So it's the best we can give.
  #[serde(rename = "overrideNextRefreshDate")]
  override_next_refresh_date: Option<String>
}

impl DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
  /// Request this component if you want the details about an item being sold in relation to the character making the request: whether the character can buy it, whether they can afford it, and other data related to purchasing the item.  Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's \"items\" property.
  pub fn new() -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
      sale_status: None,
      required_unlocks: None,
      unlock_statuses: None,
      failure_indexes: None,
      augments: None,
      vendor_item_index: None,
      item_hash: None,
      override_style_item_hash: None,
      quantity: None,
      costs: None,
      override_next_refresh_date: None
    }
  }

  pub fn set_sale_status(&mut self, sale_status: i32) {
    self.sale_status = Some(sale_status);
  }

  pub fn with_sale_status(mut self, sale_status: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.sale_status = Some(sale_status);
    self
  }

  pub fn sale_status(&self) -> Option<&i32> {
    self.sale_status.as_ref()
  }

  pub fn reset_sale_status(&mut self) {
    self.sale_status = None;
  }

  pub fn set_required_unlocks(&mut self, required_unlocks: Vec<i32>) {
    self.required_unlocks = Some(required_unlocks);
  }

  pub fn with_required_unlocks(mut self, required_unlocks: Vec<i32>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.required_unlocks = Some(required_unlocks);
    self
  }

  pub fn required_unlocks(&self) -> Option<&Vec<i32>> {
    self.required_unlocks.as_ref()
  }

  pub fn reset_required_unlocks(&mut self) {
    self.required_unlocks = None;
  }

  pub fn set_unlock_statuses(&mut self, unlock_statuses: Vec<::models::DestinyDestinyUnlockStatus>) {
    self.unlock_statuses = Some(unlock_statuses);
  }

  pub fn with_unlock_statuses(mut self, unlock_statuses: Vec<::models::DestinyDestinyUnlockStatus>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.unlock_statuses = Some(unlock_statuses);
    self
  }

  pub fn unlock_statuses(&self) -> Option<&Vec<::models::DestinyDestinyUnlockStatus>> {
    self.unlock_statuses.as_ref()
  }

  pub fn reset_unlock_statuses(&mut self) {
    self.unlock_statuses = None;
  }

  pub fn set_failure_indexes(&mut self, failure_indexes: Vec<i32>) {
    self.failure_indexes = Some(failure_indexes);
  }

  pub fn with_failure_indexes(mut self, failure_indexes: Vec<i32>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.failure_indexes = Some(failure_indexes);
    self
  }

  pub fn failure_indexes(&self) -> Option<&Vec<i32>> {
    self.failure_indexes.as_ref()
  }

  pub fn reset_failure_indexes(&mut self) {
    self.failure_indexes = None;
  }

  pub fn set_augments(&mut self, augments: i32) {
    self.augments = Some(augments);
  }

  pub fn with_augments(mut self, augments: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.augments = Some(augments);
    self
  }

  pub fn augments(&self) -> Option<&i32> {
    self.augments.as_ref()
  }

  pub fn reset_augments(&mut self) {
    self.augments = None;
  }

  pub fn set_vendor_item_index(&mut self, vendor_item_index: i32) {
    self.vendor_item_index = Some(vendor_item_index);
  }

  pub fn with_vendor_item_index(mut self, vendor_item_index: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.vendor_item_index = Some(vendor_item_index);
    self
  }

  pub fn vendor_item_index(&self) -> Option<&i32> {
    self.vendor_item_index.as_ref()
  }

  pub fn reset_vendor_item_index(&mut self) {
    self.vendor_item_index = None;
  }

  pub fn set_item_hash(&mut self, item_hash: i32) {
    self.item_hash = Some(item_hash);
  }

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_override_style_item_hash(&mut self, override_style_item_hash: i32) {
    self.override_style_item_hash = Some(override_style_item_hash);
  }

  pub fn with_override_style_item_hash(mut self, override_style_item_hash: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.override_style_item_hash = Some(override_style_item_hash);
    self
  }

  pub fn override_style_item_hash(&self) -> Option<&i32> {
    self.override_style_item_hash.as_ref()
  }

  pub fn reset_override_style_item_hash(&mut self) {
    self.override_style_item_hash = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = Some(quantity);
  }

  pub fn with_quantity(mut self, quantity: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.quantity = Some(quantity);
    self
  }

  pub fn quantity(&self) -> Option<&i32> {
    self.quantity.as_ref()
  }

  pub fn reset_quantity(&mut self) {
    self.quantity = None;
  }

  pub fn set_costs(&mut self, costs: Vec<::models::DestinyDestinyItemQuantity>) {
    self.costs = Some(costs);
  }

  pub fn with_costs(mut self, costs: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.costs = Some(costs);
    self
  }

  pub fn costs(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.costs.as_ref()
  }

  pub fn reset_costs(&mut self) {
    self.costs = None;
  }

  pub fn set_override_next_refresh_date(&mut self, override_next_refresh_date: String) {
    self.override_next_refresh_date = Some(override_next_refresh_date);
  }

  pub fn with_override_next_refresh_date(mut self, override_next_refresh_date: String) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.override_next_refresh_date = Some(override_next_refresh_date);
    self
  }

  pub fn override_next_refresh_date(&self) -> Option<&String> {
    self.override_next_refresh_date.as_ref()
  }

  pub fn reset_override_next_refresh_date(&mut self) {
    self.override_next_refresh_date = None;
  }

}


