/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsInventoryDestinyCurrenciesComponent : This component provides a quick lookup of every item the requested character has and how much of that item they have.  Requesting this component will allow you to circumvent manually putting together the list of which currencies you have for the purpose of testing currency requirements on an item being purchased, or operations that have costs.  You *could* figure this out yourself by doing a GetCharacter or GetProfile request and forming your own lookup table, but that is inconvenient enough that this feels like a worthwhile (and optional) redundency. Don't bother requesting it if you have already created your own lookup from prior GetCharacter/GetProfile calls.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsInventoryDestinyCurrenciesComponent {
  /// A dictionary - keyed by the item's hash identifier (DestinyInventoryItemDefinition), and whose value is the amount of that item you have across all available inventory buckets for purchasing.  This allows you to see whether the requesting character can afford any given purchase/action without having to re-create this list itself.
  #[serde(rename = "itemQuantities")]
  item_quantities: Option<::std::collections::HashMap<String, i32>>
}

impl DestinyComponentsInventoryDestinyCurrenciesComponent {
  /// This component provides a quick lookup of every item the requested character has and how much of that item they have.  Requesting this component will allow you to circumvent manually putting together the list of which currencies you have for the purpose of testing currency requirements on an item being purchased, or operations that have costs.  You *could* figure this out yourself by doing a GetCharacter or GetProfile request and forming your own lookup table, but that is inconvenient enough that this feels like a worthwhile (and optional) redundency. Don't bother requesting it if you have already created your own lookup from prior GetCharacter/GetProfile calls.
  pub fn new() -> DestinyComponentsInventoryDestinyCurrenciesComponent {
    DestinyComponentsInventoryDestinyCurrenciesComponent {
      item_quantities: None
    }
  }

  pub fn set_item_quantities(&mut self, item_quantities: ::std::collections::HashMap<String, i32>) {
    self.item_quantities = Some(item_quantities);
  }

  pub fn with_item_quantities(mut self, item_quantities: ::std::collections::HashMap<String, i32>) -> DestinyComponentsInventoryDestinyCurrenciesComponent {
    self.item_quantities = Some(item_quantities);
    self
  }

  pub fn item_quantities(&self) -> Option<&::std::collections::HashMap<String, i32>> {
    self.item_quantities.as_ref()
  }

  pub fn reset_item_quantities(&mut self) {
    self.item_quantities = None;
  }

}



