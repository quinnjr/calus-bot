/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyVendorCategoryEntryDefinition : This is the definition for a single Vendor Category, into which Sale Items are grouped.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
  /// The index of the category in the original category definitions for the vendor.
  #[serde(rename = "categoryIndex")]
  category_index: Option<i32>,
  /// Used in sorting items in vendors... but there's a lot more to it. Just go with the order provided in the itemIndexes property on the DestinyVendorCategoryComponent instead, it should be more reliable than trying to recalculate it yourself.
  #[serde(rename = "sortValue")]
  sort_value: Option<i32>,
  /// The hashed identifier for the category.
  #[serde(rename = "categoryHash")]
  category_hash: Option<i32>,
  /// The amount of items that will be available when this category is shown.
  #[serde(rename = "quantityAvailable")]
  quantity_available: Option<i32>,
  /// If items aren't up for sale in this category, should we still show them (greyed out)?
  #[serde(rename = "showUnavailableItems")]
  show_unavailable_items: Option<bool>,
  /// If you don't have the currency required to buy items from this category, should the items be hidden?
  #[serde(rename = "hideIfNoCurrency")]
  hide_if_no_currency: Option<bool>,
  /// True if this category doesn't allow purchases.
  #[serde(rename = "hideFromRegularPurchase")]
  hide_from_regular_purchase: Option<bool>,
  /// The localized string for making purchases from this category, if it is different from the vendor's string for purchasing.
  #[serde(rename = "buyStringOverride")]
  buy_string_override: Option<String>,
  /// If the category is disabled, this is the localized description to show.
  #[serde(rename = "disabledDescription")]
  disabled_description: Option<String>,
  /// The localized title of the category.
  #[serde(rename = "displayTitle")]
  display_title: Option<String>,
  /// If this category has an overlay prompt that should appear, this contains the details of that prompt.
  #[serde(rename = "overlay")]
  overlay: Option<Value>,
  /// A shortcut for the vendor item indexes sold under this category. Saves us from some expensive reorganization at runtime.
  #[serde(rename = "vendorItemIndexes")]
  vendor_item_indexes: Option<Vec<i32>>,
  /// Sometimes a category isn't actually used to sell items, but rather to preview them. This implies different UI (and manual placement of the category in the UI) in the game, and special treatment.
  #[serde(rename = "isPreview")]
  is_preview: Option<bool>,
  /// If true, this category only displays items: you can't purchase anything in them.
  #[serde(rename = "isDisplayOnly")]
  is_display_only: Option<bool>,
  #[serde(rename = "resetIntervalMinutesOverride")]
  reset_interval_minutes_override: Option<i32>,
  #[serde(rename = "resetOffsetMinutesOverride")]
  reset_offset_minutes_override: Option<i32>
}

impl DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
  /// This is the definition for a single Vendor Category, into which Sale Items are grouped.
  pub fn new() -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
      category_index: None,
      sort_value: None,
      category_hash: None,
      quantity_available: None,
      show_unavailable_items: None,
      hide_if_no_currency: None,
      hide_from_regular_purchase: None,
      buy_string_override: None,
      disabled_description: None,
      display_title: None,
      overlay: None,
      vendor_item_indexes: None,
      is_preview: None,
      is_display_only: None,
      reset_interval_minutes_override: None,
      reset_offset_minutes_override: None
    }
  }

  pub fn set_category_index(&mut self, category_index: i32) {
    self.category_index = Some(category_index);
  }

  pub fn with_category_index(mut self, category_index: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.category_index = Some(category_index);
    self
  }

  pub fn category_index(&self) -> Option<&i32> {
    self.category_index.as_ref()
  }

  pub fn reset_category_index(&mut self) {
    self.category_index = None;
  }

  pub fn set_sort_value(&mut self, sort_value: i32) {
    self.sort_value = Some(sort_value);
  }

  pub fn with_sort_value(mut self, sort_value: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.sort_value = Some(sort_value);
    self
  }

  pub fn sort_value(&self) -> Option<&i32> {
    self.sort_value.as_ref()
  }

  pub fn reset_sort_value(&mut self) {
    self.sort_value = None;
  }

  pub fn set_category_hash(&mut self, category_hash: i32) {
    self.category_hash = Some(category_hash);
  }

  pub fn with_category_hash(mut self, category_hash: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.category_hash = Some(category_hash);
    self
  }

  pub fn category_hash(&self) -> Option<&i32> {
    self.category_hash.as_ref()
  }

  pub fn reset_category_hash(&mut self) {
    self.category_hash = None;
  }

  pub fn set_quantity_available(&mut self, quantity_available: i32) {
    self.quantity_available = Some(quantity_available);
  }

  pub fn with_quantity_available(mut self, quantity_available: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.quantity_available = Some(quantity_available);
    self
  }

  pub fn quantity_available(&self) -> Option<&i32> {
    self.quantity_available.as_ref()
  }

  pub fn reset_quantity_available(&mut self) {
    self.quantity_available = None;
  }

  pub fn set_show_unavailable_items(&mut self, show_unavailable_items: bool) {
    self.show_unavailable_items = Some(show_unavailable_items);
  }

  pub fn with_show_unavailable_items(mut self, show_unavailable_items: bool) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.show_unavailable_items = Some(show_unavailable_items);
    self
  }

  pub fn show_unavailable_items(&self) -> Option<&bool> {
    self.show_unavailable_items.as_ref()
  }

  pub fn reset_show_unavailable_items(&mut self) {
    self.show_unavailable_items = None;
  }

  pub fn set_hide_if_no_currency(&mut self, hide_if_no_currency: bool) {
    self.hide_if_no_currency = Some(hide_if_no_currency);
  }

  pub fn with_hide_if_no_currency(mut self, hide_if_no_currency: bool) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.hide_if_no_currency = Some(hide_if_no_currency);
    self
  }

  pub fn hide_if_no_currency(&self) -> Option<&bool> {
    self.hide_if_no_currency.as_ref()
  }

  pub fn reset_hide_if_no_currency(&mut self) {
    self.hide_if_no_currency = None;
  }

  pub fn set_hide_from_regular_purchase(&mut self, hide_from_regular_purchase: bool) {
    self.hide_from_regular_purchase = Some(hide_from_regular_purchase);
  }

  pub fn with_hide_from_regular_purchase(mut self, hide_from_regular_purchase: bool) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.hide_from_regular_purchase = Some(hide_from_regular_purchase);
    self
  }

  pub fn hide_from_regular_purchase(&self) -> Option<&bool> {
    self.hide_from_regular_purchase.as_ref()
  }

  pub fn reset_hide_from_regular_purchase(&mut self) {
    self.hide_from_regular_purchase = None;
  }

  pub fn set_buy_string_override(&mut self, buy_string_override: String) {
    self.buy_string_override = Some(buy_string_override);
  }

  pub fn with_buy_string_override(mut self, buy_string_override: String) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.buy_string_override = Some(buy_string_override);
    self
  }

  pub fn buy_string_override(&self) -> Option<&String> {
    self.buy_string_override.as_ref()
  }

  pub fn reset_buy_string_override(&mut self) {
    self.buy_string_override = None;
  }

  pub fn set_disabled_description(&mut self, disabled_description: String) {
    self.disabled_description = Some(disabled_description);
  }

  pub fn with_disabled_description(mut self, disabled_description: String) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.disabled_description = Some(disabled_description);
    self
  }

  pub fn disabled_description(&self) -> Option<&String> {
    self.disabled_description.as_ref()
  }

  pub fn reset_disabled_description(&mut self) {
    self.disabled_description = None;
  }

  pub fn set_display_title(&mut self, display_title: String) {
    self.display_title = Some(display_title);
  }

  pub fn with_display_title(mut self, display_title: String) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.display_title = Some(display_title);
    self
  }

  pub fn display_title(&self) -> Option<&String> {
    self.display_title.as_ref()
  }

  pub fn reset_display_title(&mut self) {
    self.display_title = None;
  }

  pub fn set_overlay(&mut self, overlay: Value) {
    self.overlay = Some(overlay);
  }

  pub fn with_overlay(mut self, overlay: Value) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.overlay = Some(overlay);
    self
  }

  pub fn overlay(&self) -> Option<&Value> {
    self.overlay.as_ref()
  }

  pub fn reset_overlay(&mut self) {
    self.overlay = None;
  }

  pub fn set_vendor_item_indexes(&mut self, vendor_item_indexes: Vec<i32>) {
    self.vendor_item_indexes = Some(vendor_item_indexes);
  }

  pub fn with_vendor_item_indexes(mut self, vendor_item_indexes: Vec<i32>) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.vendor_item_indexes = Some(vendor_item_indexes);
    self
  }

  pub fn vendor_item_indexes(&self) -> Option<&Vec<i32>> {
    self.vendor_item_indexes.as_ref()
  }

  pub fn reset_vendor_item_indexes(&mut self) {
    self.vendor_item_indexes = None;
  }

  pub fn set_is_preview(&mut self, is_preview: bool) {
    self.is_preview = Some(is_preview);
  }

  pub fn with_is_preview(mut self, is_preview: bool) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.is_preview = Some(is_preview);
    self
  }

  pub fn is_preview(&self) -> Option<&bool> {
    self.is_preview.as_ref()
  }

  pub fn reset_is_preview(&mut self) {
    self.is_preview = None;
  }

  pub fn set_is_display_only(&mut self, is_display_only: bool) {
    self.is_display_only = Some(is_display_only);
  }

  pub fn with_is_display_only(mut self, is_display_only: bool) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.is_display_only = Some(is_display_only);
    self
  }

  pub fn is_display_only(&self) -> Option<&bool> {
    self.is_display_only.as_ref()
  }

  pub fn reset_is_display_only(&mut self) {
    self.is_display_only = None;
  }

  pub fn set_reset_interval_minutes_override(&mut self, reset_interval_minutes_override: i32) {
    self.reset_interval_minutes_override = Some(reset_interval_minutes_override);
  }

  pub fn with_reset_interval_minutes_override(mut self, reset_interval_minutes_override: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.reset_interval_minutes_override = Some(reset_interval_minutes_override);
    self
  }

  pub fn reset_interval_minutes_override(&self) -> Option<&i32> {
    self.reset_interval_minutes_override.as_ref()
  }

  pub fn reset_reset_interval_minutes_override(&mut self) {
    self.reset_interval_minutes_override = None;
  }

  pub fn set_reset_offset_minutes_override(&mut self, reset_offset_minutes_override: i32) {
    self.reset_offset_minutes_override = Some(reset_offset_minutes_override);
  }

  pub fn with_reset_offset_minutes_override(mut self, reset_offset_minutes_override: i32) -> DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    self.reset_offset_minutes_override = Some(reset_offset_minutes_override);
    self
  }

  pub fn reset_offset_minutes_override(&self) -> Option<&i32> {
    self.reset_offset_minutes_override.as_ref()
  }

  pub fn reset_reset_offset_minutes_override(&mut self) {
    self.reset_offset_minutes_override = None;
  }

}



