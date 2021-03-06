/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyVendorAcceptedItemDefinition : If you ever wondered how the Vault works, here it is.  The Vault is merely a set of inventory buckets that exist on your Profile/Account level. When you transfer items in the Vault, the game is using the Vault Vendor's DestinyVendorAcceptedItemDefinitions to see where the appropriate destination bucket is for the source bucket from whence your item is moving. If it finds such an entry, it transfers the item to the other bucket.  The mechanics for Postmaster works similarly, which is also a vendor. All driven by Accepted Items.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
  /// The \"source\" bucket for a transfer. When a user wants to transfer an item, the appropriate DestinyVendorDefinition's acceptedItems property is evaluated, looking for an entry where acceptedInventoryBucketHash matches the bucket that the item being transferred is currently located. If it exists, the item will be transferred into whatever bucket is defined by destinationInventoryBucketHash.
  #[serde(rename = "acceptedInventoryBucketHash")]
  accepted_inventory_bucket_hash: Option<i32>,
  /// This is the bucket where the item being transferred will be put, given that it was being transferred *from* the bucket defined in acceptedInventoryBucketHash.
  #[serde(rename = "destinationInventoryBucketHash")]
  destination_inventory_bucket_hash: Option<i32>
}

impl DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
  /// If you ever wondered how the Vault works, here it is.  The Vault is merely a set of inventory buckets that exist on your Profile/Account level. When you transfer items in the Vault, the game is using the Vault Vendor's DestinyVendorAcceptedItemDefinitions to see where the appropriate destination bucket is for the source bucket from whence your item is moving. If it finds such an entry, it transfers the item to the other bucket.  The mechanics for Postmaster works similarly, which is also a vendor. All driven by Accepted Items.
  pub fn new() -> DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
    DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
      accepted_inventory_bucket_hash: None,
      destination_inventory_bucket_hash: None
    }
  }

  pub fn set_accepted_inventory_bucket_hash(&mut self, accepted_inventory_bucket_hash: i32) {
    self.accepted_inventory_bucket_hash = Some(accepted_inventory_bucket_hash);
  }

  pub fn with_accepted_inventory_bucket_hash(mut self, accepted_inventory_bucket_hash: i32) -> DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
    self.accepted_inventory_bucket_hash = Some(accepted_inventory_bucket_hash);
    self
  }

  pub fn accepted_inventory_bucket_hash(&self) -> Option<&i32> {
    self.accepted_inventory_bucket_hash.as_ref()
  }

  pub fn reset_accepted_inventory_bucket_hash(&mut self) {
    self.accepted_inventory_bucket_hash = None;
  }

  pub fn set_destination_inventory_bucket_hash(&mut self, destination_inventory_bucket_hash: i32) {
    self.destination_inventory_bucket_hash = Some(destination_inventory_bucket_hash);
  }

  pub fn with_destination_inventory_bucket_hash(mut self, destination_inventory_bucket_hash: i32) -> DestinyDefinitionsDestinyVendorAcceptedItemDefinition {
    self.destination_inventory_bucket_hash = Some(destination_inventory_bucket_hash);
    self
  }

  pub fn destination_inventory_bucket_hash(&self) -> Option<&i32> {
    self.destination_inventory_bucket_hash.as_ref()
  }

  pub fn reset_destination_inventory_bucket_hash(&mut self) {
    self.destination_inventory_bucket_hash = None;
  }

}



